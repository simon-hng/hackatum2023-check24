"use client";
import { MapPinIcon } from "@heroicons/react/20/solid";
import { useState } from "react";
import { proxyApi } from "~/lib/api";
import { Craftsman } from "~/lib/types/craftsman";

interface Props {
  craftsmen: Craftsman[];
  postalcode: string;
}

export const CraftsmenList = (props: Props) => {
  // We're starting with the second page since the first is passed
  const [page, setPage] = useState(2);
  const [craftsmen, setCraftsmen] = useState(props.craftsmen);

  const loadmore = async () => {
    const newCraftsmen = await proxyApi
      .get("api/craftsmen", {
        params: {
          postalcode: props.postalcode,
          page,
        },
      })
      .then((res) => res.data as Craftsman[]);

    setCraftsmen([...craftsmen, ...newCraftsmen]);

    setPage(page + 1);
  };

  return (
    <>
      <div className="grid gap-3 px-4 sm:px-16 md:grid-cols-2">
        {craftsmen.map((craftsman, i) => (
          <div
            key={craftsman.id}
            className="card flex w-full flex-row items-center bg-base-100 px-8 py-4 shadow-xl"
          >
            <figure>
              <div className="avatar">
                <div className="w-24 rounded-full">
                  <img alt="handwerker" src={`/handwerker/${i % 12}.jpeg`} />
                </div>
              </div>
            </figure>
            <div className="card-body pr-0">
              <h2 className="card-title">
                {craftsman.first_name} {craftsman.last_name}
              </h2>
              <div className="flex gap-4">
                <p className="flex items-center gap-2">
                  <MapPinIcon className="-ml-1 h-6 w-6" />
                  {Math.round(craftsman.distance * 10) / 10} km entfernt
                </p>
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
        <button className="btn mx-auto w-96" onClick={loadmore}>
          Mehr Anzeigen
        </button>
      </div>
    </>
  );
};
