import { useEffect, useState } from "react";
import { QUERY_SERVICE_ENDPOINT } from "../utils/constants";
import { CommentData } from "./CommentList";
import { Post } from "./Post";

export interface PostData {
  id: string;
  title: string;
  comments: CommentData[];
}

export const PostList = () => {
  const [posts, setPosts] = useState<PostData[]>([]);

  const fetchPosts = async () => {
    const res = await fetch(QUERY_SERVICE_ENDPOINT, { method: "GET" });
    const data: { [id: string]: PostData } = await res.json();
    console.log(data);
    const posts = Object.values(data).map((post) => post);
    setPosts(posts);
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
