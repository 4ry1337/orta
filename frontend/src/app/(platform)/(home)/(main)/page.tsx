"use client";

import { useSession } from "@/context/session_context";

const Home = () => {
  const { status } = useSession();
  return <div>Hello</div>;
};

export default Home;
