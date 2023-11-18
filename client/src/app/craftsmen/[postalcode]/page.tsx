import { api } from "~/lib/api";
import { Craftsman } from "~/lib/types/craftsman";
import { CraftsmenList } from "./_components/craftsmenList";

interface PageProps {
  params: {
    postalcode: string;
  };
}

export default async function Page({ params }: PageProps) {
  let page = 1;
  const initialCraftsmen: Craftsman[] = await api
    .get("/craftsmen", {
      params: {
        postalcode: params.postalcode,
        page,
      },
    })
    .then((res) => res.data);

  return (
    <section>
      <CraftsmenList
        craftsmen={initialCraftsmen}
        postalcode={params.postalcode}
      />
    </section>
  );
}
