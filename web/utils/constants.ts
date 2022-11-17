export const POST_SERVICE_ENDPOINT = "http://localhost:4000/posts";
export const COMMENT_SERVICE_ENDPOINT = (postId: string) =>
  `http://localhost:4001/posts/${postId}/comments`;

export const QUERY_SERVICE_ENDPOINT = "http://localhost:4002/posts";
