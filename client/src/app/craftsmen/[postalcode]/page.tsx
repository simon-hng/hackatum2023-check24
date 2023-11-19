import { api } from "~/lib/api";
import { Craftsman } from "~/lib/types/craftsman";
import { CraftsmenList } from "./_components/craftsmenList";

interface PageProps {
  params: {
    postalcode: string;
  };
}

export default async function Page({ params }: PageProps) {
  const initialCraftsmen = await api
    .get("/craftsmen", {
      params: {
        postalcode: params.postalcode,
        page: 1,
      },
    })
    .then((res) => res.data as Craftsman[]);

  return (
    <section>
      <CraftsmenList
        craftsmen={initialCraftsmen}
        postalcode={params.postalcode}
      />
    </section>
  );
}
