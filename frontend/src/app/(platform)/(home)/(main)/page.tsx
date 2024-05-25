"use client";

import Image from "next/image";

const Home = () => {
  return (
    <div className="w-full">
      <div>
        <Image
          alt="image"
          src={"http://localhost:5000/api/assets/25-05-2024_11:30:32_erd.png"}
          fill
        />
      </div>
      <div>Hello</div>
    </div>
  );
};

export default Home;
