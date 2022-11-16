import Head from "next/head";
import { CreatePost } from "../components/CreatePost";
import { PostList } from "../components/PostList";

export default function Home() {
  return (
    <div>
      <Head>
        <title>My microservice blog</title>
      </Head>
      <div className="p-32">
        <h1 className="text-6xl font-bold">
          Blog, powered with{" "}
          <span className="italic text-blue-700">microservice</span>
        </h1>
        <div className="my-12 w-1/2">
          <CreatePost />
        </div>
        <div className="my-12">
          <PostList />
        </div>
      </div>
    </div>
  );
}
