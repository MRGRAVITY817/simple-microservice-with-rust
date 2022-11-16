import { useEffect, useState } from "react";
import { POST_SERVICE_ENDPOINT } from "../utils/constants";
import { Post } from "./Post";

export interface PostData {
  id: string;
  title: string;
}

export const PostList = () => {
  const [posts, setPosts] = useState<PostData[]>([]);

  const fetchPosts = async () => {
    const res = await fetch(POST_SERVICE_ENDPOINT, { method: "GET" });
    const data: { posts: PostData[] } = await res.json();

    setPosts(data.posts);
  };

  useEffect(() => {
    fetchPosts();
  }, []);

  return (
    <div>
      <h2 className="text-3xl font-medium mb-8">Posts</h2>
      <div className="grid gap-4 grid-cols-3">
        {posts.map((post) => (
          <Post key={post.id} {...post} />
        ))}
      </div>
    </div>
  );
};
