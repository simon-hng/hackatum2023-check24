import { api } from "~/lib/api";
import { Craftsman } from "~/lib/types/craftsman";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const page = searchParams.get("page");
  const postalcode = searchParams.get("postalcode");

  const craftsmen = await api
    .get("/craftsmen", {
      params: {
        postalcode: postalcode,
        page,
      },
    })
    .then((res) => res.data as Craftsman[]);

  return Response.json(craftsmen);
}
