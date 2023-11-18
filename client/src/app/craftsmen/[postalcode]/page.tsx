import { faker } from "@faker-js/faker";
import { api } from "~/lib/api";
import { Craftsman } from "~/lib/types/craftsman";

interface PageProps {
  params: {
    postalcode: string;
  };
}

export default async function Page({ params }: PageProps) {
  const craftsmen: Craftsman[] = await api
    .get("/craftsmen", {
      params: {
        postalcode: params.postalcode,
      },
    })
    .then((res) => res.data);

  return (
    <section>
      <div className="grid grid-cols-2 gap-3 px-16">
        {craftsmen.map((craftsman) => (
          <div key={craftsman.id} className="card w-full bg-base-100 shadow-xl">
            <figure className="px-10 pt-10">
              <div className="avatar">
                <div className="w-24 rounded-full">
                  <img src={faker.image.avatar()} />
                </div>
              </div>
            </figure>
            <div className="card-body">
              <h2 className="card-title">
                {craftsman.first_name} {craftsman.last_name}
              </h2>
              <div className="flex gap-4">
                <p>{Math.round(craftsman.distance * 10) / 10} km entfernt</p>
                <p className="text-right">{craftsman.rank} rank</p>
              </div>
              <div className="flex gap-4">
                <p>{craftsman.profile_picture_score} picture score</p>
                <p className="text-right">
                  {craftsman.profile_description_score} description
                </p>
              </div>
            </div>
          </div>
        ))}
      </div>
      <div className="flex w-full items-center py-4">
        <button className="btn mx-auto w-96">Mehr Anzeigen</button>
      </div>
    </section>
  );
}
