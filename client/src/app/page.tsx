"use client";
import { useState } from "react";
import { useRouter } from "next/navigation";

export default function HomePage() {
  const [postalcode, setPostalcode] = useState("");
  const router = useRouter();

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    router.push(`/craftsmen/${postalcode}`);
  };

  return (
    <main>
      <section className="mx-auto max-w-md py-16">
        <form className="flex flex-col gap-3" onSubmit={handleSubmit}>
          <div className="flex flex-col gap-2">
            <h1 className="text-large text-info">
              In welcher plz suchen Sie einen Handwerker?
            </h1>
            <input
              onChange={(e) => setPostalcode(e.target.value)}
              type="text"
              placeholder="plz"
              className="input input-bordered w-full"
            />
          </div>
          <button
            type="submit"
            className="btn bg-neutral text-white hover:bg-secondary"
          >
            Weiter
          </button>
        </form>
      </section>
    </main>
  );
}
