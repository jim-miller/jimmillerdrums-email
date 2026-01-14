const AWS = require('aws-sdk');
const ses = new AWS.SES({ region: process.env.AWS_REGION });
const s3 = new AWS.S3({ region: process.env.AWS_REGION });

exports.handler = async (event) => {
    console.log('Received SES event:', JSON.stringify(event, null, 2));
    
    try {
        const sesRecord = event.Records[0].ses;
        const messageId = sesRecord.mail.messageId;
        const source = sesRecord.mail.source;
        const destination = sesRecord.mail.destination[0];
        
        console.log(`Processing email: ${messageId} from ${source} to ${destination}`);
        
        const forwardToEmail = process.env.FORWARD_TO_EMAIL;
        if (!forwardToEmail) {
            throw new Error('FORWARD_TO_EMAIL environment variable not set');
        }
        
        // Get the original email from S3
        const s3Key = `incoming/${messageId}`;
        console.log(`Retrieving email from S3: ${s3Key}`);
        
        const s3Object = await s3.getObject({
            Bucket: process.env.EMAIL_BUCKET,
            Key: s3Key
        }).promise();
        
        const originalEmail = s3Object.Body.toString();
        console.log('Retrieved original email from S3');
        
        // Extract original email details
        const subjectMatch = originalEmail.match(/^Subject: (.*)$/m);
        const subject = subjectMatch ? subjectMatch[1] : 'Forwarded Email';
        
        const fromMatch = originalEmail.match(/^From: (.*)$/m);
        const originalFrom = fromMatch ? fromMatch[1] : source;
        
        // Extract email body and parse MIME content
        const bodyStart = originalEmail.indexOf('\r\n\r\n');
        const rawBody = bodyStart !== -1 ? originalEmail.substring(bodyStart + 4) : originalEmail;
        
        // Extract plain text from multipart email
        let cleanBody = rawBody;
        if (rawBody.includes('Content-Type: text/plain')) {
            const textMatch = rawBody.match(/Content-Type: text\/plain[^]*?\r?\n\r?\n([^]*?)(?=\r?\n--|\r?\n$)/);
            if (textMatch) {
                cleanBody = textMatch[1]
                    .replace(/=\r?\n/g, '') // Remove quoted-printable line breaks
                    .replace(/=([0-9A-F]{2})/g, (match, hex) => String.fromCharCode(parseInt(hex, 16))); // Decode quoted-printable
            }
        }
        
        const params = {
            Source: 'jim@jimmillerdrums.com',
            Destination: {
                ToAddresses: [forwardToEmail]
            },
            ReplyToAddresses: [source],
            Message: {
                Subject: {
                    Data: subject,
                    Charset: 'UTF-8'
                },
                Body: {
                    Text: {
                        Data: cleanBody.trim(),
                        Charset: 'UTF-8'
                    }
                }
            }
        };
        
        const result = await ses.sendEmail(params).promise();
        console.log('Email forwarded successfully:', result.MessageId);
        
        return {
            statusCode: 200,
            body: JSON.stringify({
                message: 'Email forwarded successfully',
                originalMessageId: messageId,
                forwardedMessageId: result.MessageId
            })
        };
        
    } catch (error) {
        console.error('Error processing email:', error);
        throw error;
    }
};
