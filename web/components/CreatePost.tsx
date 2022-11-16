import { FormEvent, useState } from "react";
import { POST_SERVICE_ENDPOINT } from "../utils/constants";

export const CreatePost = () => {
  const [title, setTitle] = useState<string>("");

  const createPost = async (e: FormEvent) => {
    e.preventDefault();
    if (title.length > 0) {
      const res = await fetch(POST_SERVICE_ENDPOINT, {
        method: "POST",
        body: JSON.stringify({ title }),
        headers: {
          "Content-Type": "application/json",
        },
      });

      console.log(res);
    }
  };

  return (
    <form onSubmit={createPost} className="flex flex-col">
      <h2 className="text-3xl font-medium mb-8">Create new post</h2>
      <label className="text-xl mb-2">Post Title</label>
      <input
        type="text"
        className="rounded-md border-2 focus:outline-none border-gray-400 hover:border-gray-500 focus:border-gray-500 p-2"
        onChange={(e) => setTitle(e.target.value)}
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
