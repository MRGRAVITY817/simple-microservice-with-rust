import Head from "next/head"

export default function Home() {
  return (
    <div>
      <Head>
        <title>My microservice blog</title>
      </Head>
      <div className="p-32">
        <h1 className="text-4xl">Blog</h1>
      </div>
    </div>
  )
}
