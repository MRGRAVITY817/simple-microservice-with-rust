import { FormEvent, useState } from "react";
import { POST_SERVICE_ENDPOINT } from "../utils/constants";

export const CreatePost = () => {
  const [title, setTitle] = useState<string>("");

  const createPost = async (e: FormEvent) => {
    e.preventDefault();
    if (title.length > 0) {
      await fetch(POST_SERVICE_ENDPOINT, {
        method: "POST",
        body: JSON.stringify({ title }),
        headers: {
          "Content-Type": "application/json",
        },
      });

      setTitle("");
    }
  };

  return (
    <form onSubmit={createPost} className="flex flex-col">
      <h2 className="text-3xl font-medium mb-8">Create new post</h2>
      <input
        type="text"
        placeholder="Enter post title..."
        className="rounded-md border-2 focus:outline-none border-gray-300 hover:border-gray-500 focus:border-gray-500 p-2 text-xl"
        onChange={(e) => setTitle(e.target.value)}
        value={title}
      />
      <button
        type="submit"
        className="text-xl text-white font-semibold mt-4 bg-blue-700 py-2 rounded-md hover:bg-blue-800"
      >
        Create
      </button>
    </form>
  );
};
