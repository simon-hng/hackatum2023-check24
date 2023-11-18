"use client";

import { LatLngTuple } from "leaflet";
import { Marker, Popup, TileLayer } from "react-leaflet";
import { MapContainer } from "react-leaflet/MapContainer";
import { Craftsman } from "~/lib/types/craftsman";

interface Props {
  craftsmen?: Craftsman[];
}

export const CraftsmanMap = ({ craftsmen }: Props) => {
  const center = [
    craftsmen?.at(0)?.lat ?? 0,
    craftsmen?.at(0)?.lon ?? 0,
  ] as LatLngTuple;

  return (
    <div className="h-32 flex">
      <MapContainer center={center} zoom={13} scrollWheelZoom={false}>
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        <Marker position={center}>
          <Popup>
            A pretty CSS3 popup. <br /> Easily customizable.
          </Popup>
        </Marker>
      </MapContainer>
    </div>
  );
};
