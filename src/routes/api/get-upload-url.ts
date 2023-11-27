import { handleUpload, type HandleUploadBody } from '@vercel/blob/client';

export async function post({ request }) {
	const body = await request.json();

	try {
		const jsonResponse = await handleUpload({
			body,
			request,
			onBeforeGenerateToken: async (pathname, clientPayload) => {
				return {
					allowedContentTypes: ['audio/*'],
					// validUntil: number, optional, timestamp in ms, by default now + 30s (30,000)
					tokenPayload: JSON.stringify({
						// optional, sent to your server on upload completion
					})
				};
			},
			onUploadCompleted: async ({ blob, tokenPayload }) => {
				console.log('blob upload completed', blob, tokenPayload);
				try {
					// Run any logic after the file upload completed
				} catch (error) {
					throw new Error('Could not update post');
				}
			}
		});

		return new Response(JSON.stringify(jsonResponse));
	} catch (error) {
		return new Response(
			JSON.stringify({ error: error.message }),
			{ status: 400 } // The webhook will retry 5 times waiting for a 200
		);
	}
}
