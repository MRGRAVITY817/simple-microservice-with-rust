import { FC } from "react";

export interface CommentData {
  id: string;
  content: string;
  status: CommentStatus;
}

export enum CommentStatus {
  Pending = "Pending",
  Approved = "Approved",
  Rejected = "Rejected",
}

export const CommentList: FC<{ comments: CommentData[] }> = ({ comments }) => {
  return (
    <div>
      <h3 className="text-xl font-medium mb-2">Comments</h3>
      <ul>
        {comments.map((comment) => {
          const content =
            comment.status === CommentStatus.Rejected
              ? "THIS COMMENT IS REJECTED"
              : comment.content;
          return <li key={comment.id}>• {content}</li>;
        })}
      </ul>
    </div>
  );
};
