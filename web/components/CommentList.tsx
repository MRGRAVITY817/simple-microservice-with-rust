import { FC } from "react";

export interface CommentData {
  id: string;
  content: string;
}

export const CommentList: FC<{ comments: CommentData[] }> = ({ comments }) => {
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
