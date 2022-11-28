import { FC, FormEvent, useState } from "react";
import { COMMENT_SERVICE_ENDPOINT } from "../utils/constants";
import { CommentList } from "./CommentList";
import { PostData } from "./PostList";

export const Post: FC<PostData> = ({ id, title, comments }) => {
  const [comment, setComment] = useState<string>("");
  const createComment = async (e: FormEvent) => {
    e.preventDefault();
    if (comment.length > 0) {
      await fetch(COMMENT_SERVICE_ENDPOINT(id), {
        method: "POST",
        body: JSON.stringify({ content: comment }),
        headers: {
          "Content-Type": "application/json",
        },
      });

      setComment("");
    }
  };
  return (
    <div className="border-2 border-gray-400 p-6 rounded-md">
      <h3 className="text-2xl font-medium">{title}</h3>
      <form onSubmit={createComment}>
        <input
          type="text"
          placeholder="Leave a comment..."
          className="mt-4 border-2 border-gray-300 hover:border-gray-500 focus:border-gray-500 focus:outline-none rounded-md p-2 text-xl mr-4"
          value={comment}
          onChange={(e) => setComment(e.target.value)}
        />
        <button
          type="submit"
          className="text-xl text-white font-semibold mt-4 bg-blue-700 py-2 px-4 rounded-md hover:bg-blue-800"
        >
          Submit
        </button>
        <div className="mt-4">
          <CommentList comments={comments} />
        </div>
      </form>
    </div>
  );
};
