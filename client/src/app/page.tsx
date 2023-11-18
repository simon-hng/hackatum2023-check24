"use client";
import Link from "next/link";
import { useState } from "react";

export default function HomePage() {
  const [postalcode, setPostalcode] = useState("");
  return (
    <main>
      <section className="mx-auto flex max-w-md flex-col gap-3 py-16">
        <div className="flex flex-col gap-2">
          <h1 className="text-large">
            In welcher plz suchen Sie einen Handwerker?
          </h1>
          <input
            onChange={(e) => setPostalcode(e.target.value)}
            type="text"
            placeholder="plz"
            className="input input-bordered w-full"
          />
        </div>
        <Link href={`/craftsmen/${postalcode}`} className="btn">
          Weiter
        </Link>
      </section>
    </main>
  );
}
