import type { RequestHandler } from '@sveltejs/kit';
import { put } from '@vercel/blob';
import { handleUpload, type HandleUploadBody } from '@vercel/blob/client';

export const POST: RequestHandler = async ({ request }) => {
	try {
		const data = await request.json();
		const { filename, contentType, file } = data;

		if (!filename || !contentType) {
			// Missing data
			return {
				status: 400,
				body: { error: 'Missing filename or contentType' }
			};
		}

		// Generate a signed URL (mock example)
		// const signedUrl = `https://example.com/upload/${encodeURIComponent(filename)}`;

		const blob = await put(filename, file, {
			contentType,
			access: 'public'
		});

		return {
			status: 200,
			body: { url: blob.url }
		};
	} catch (error) {
		return {
			status: 400,
			body: { error: error }
		};
	}
};
