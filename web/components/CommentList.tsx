import { FC, useEffect, useState } from "react";
import { COMMENT_SERVICE_ENDPOINT } from "../utils/constants";

interface CommentData {
  id: string;
  content: string;
}

export const CommentList: FC<{ postId: string }> = ({ postId }) => {
  const [comments, setComments] = useState<CommentData[]>([]);

  const fetchComments = async () => {
    const res = await fetch(COMMENT_SERVICE_ENDPOINT(postId));
    if (res.status === 200) {
      const data = await res.json();
      setComments(data);
    }
  };

  useEffect(() => {
    fetchComments();
  }, []);
  return (
    <div>
      <h3 className="text-xl font-medium mb-2">Comments</h3>
      <ul>
        {comments.map((comment) => (
          <li key={comment.id}>• {comment.content}</li>
        ))}
      </ul>
    </div>
  );
};
