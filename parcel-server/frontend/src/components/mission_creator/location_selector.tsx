import { useEffect, useState } from "react";
import { QpidArea } from "../../api_types";
import { distanceBetween } from "../../utils/table_value_formatters/vector_math";

interface Props {
  locations: QpidArea[];
  value?: QpidArea;
  onChange?: (newValue: QpidArea) => void;
  referenceLocation?: QpidArea;
}

const LocationSelector = ({
  locations,
  value,
  onChange,
  referenceLocation,
}: Props) => {
  const [sortedLocations, setSortedLocations] = useState<QpidArea[]>([]);
  const [distances, setDistances] = useState<Record<number, number> | null>(
    null
  );

  useEffect(() => {
    // update distances if reference location is set
    let newDistances: Record<number, number> | null = null;
    if (referenceLocation != null) {
      newDistances = locations.reduce((distances, location) => {
        const distance = distanceBetween(
          location.metadata.location,
          referenceLocation?.metadata.location
        );
        distances[location.qpidId] = distance;

        return distances;
      }, {});

      setDistances(newDistances);
    } else {
      setDistances(null);
    }

    // sort by distance from reference location if set, otherwise by name
    if (referenceLocation != null) {
      const sorted = locations.sort((a, b) => {
        const aDist = newDistances[a.qpidId] ?? 0;
        const bDist = newDistances[b.qpidId] ?? 0;

        return aDist - bDist;
      });

      setSortedLocations(sorted);
    } else {
      setSortedLocations(
        locations.sort((a, b) =>
          a.names["en-us"].localeCompare(b.names["en-us"])
        )
      );
    }
  }, [locations, referenceLocation]);

  function getLocationName(qpidArea: QpidArea): string {
    // include distance from ref location if set
    if (distances != null) {
      const distance = distances[qpidArea.qpidId];
      return `${qpidArea.names["en-us"]} (${distance.toFixed(0)}m)`;
    } else {
      return qpidArea.names["en-us"];
    }
  }

  function internalOnChange(ev: React.ChangeEvent<HTMLSelectElement>) {
    const qpidId = parseInt(ev.target.value);

    if (qpidId == 0) {
      onChange(null);
      return;
    }

    const qpidArea = locations.find((a) => a.qpidId === qpidId);

    if (qpidArea == null) {
      throw new Error("Unexpected qpid id");
    }

    if (onChange != null) {
      onChange(qpidArea);
    }
  }

  return (
    <select onChange={internalOnChange} value={value?.qpidId.toString() ?? "0"}>
      <option value="0">Select a location</option>
      {sortedLocations.map((location) => (
        <option key={location.qpidId} value={location.qpidId.toString()}>
          {getLocationName(location)}
        </option>
      ))}
    </select>
  );
};

export default LocationSelector;
