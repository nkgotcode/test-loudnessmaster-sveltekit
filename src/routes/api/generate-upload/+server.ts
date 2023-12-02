import { AWS_ACCESS_KEY_ID, AWS_REGION, AWS_SECRET_ACCESS_KEY } from '$env/static/private';
import type { RequestHandler } from '@sveltejs/kit';
import { put } from '@vercel/blob';
import AWS from 'aws-sdk';

export const POST: RequestHandler = async ({ request }) => {
	const bucketName = 'loudness-master';
	const region = AWS_REGION;
	const accessKeyId = AWS_ACCESS_KEY_ID;
	const secretAccessKey = AWS_SECRET_ACCESS_KEY;
	const s3 = new AWS.S3({ region, accessKeyId, secretAccessKey, signatureVersion: 'v4' });
	try {
		const { searchParams } = new URL(request.url);
		const filename = searchParams.get('filename');
		const params = {
			Bucket: bucketName,
			Key: filename,
			Expires: 60
		};
		const presignedUrl = await s3.getSignedUrlPromise('putObject', params);
		return new Response(JSON.stringify({ url: presignedUrl }));
	} catch (error) {
		return new Response(JSON.stringify({ error: error.message }), { status: 500 });
	}
};
