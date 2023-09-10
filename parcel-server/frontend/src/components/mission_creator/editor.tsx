import { useEffect, useState } from "react";
import {
  Area,
  Baggage,
  ConstructionPointType,
  LocalizedBaggageData,
  QpidArea,
} from "../../api_types";
import * as Form from "../form";
import {
  getLostBaggages,
  getQpidAreas,
} from "../../services/game_data_service";

interface Props {
  area: Area;
  startQpidId?: number;
}

const allowedConstructionTypes: ConstructionPointType[] = [
  "deliveryBase",
  "preppersShelter",
];

const MissionEditor = ({ area, startQpidId: defaultQpidId }: Props) => {
  const [loading, setLoading] = useState(false);
  const [qpidAreas, setQpidAreas] = useState<QpidArea[] | null | undefined>(
    undefined
  );
  const [lostBaggages, setLostBaggages] = useState<
    Record<number, LocalizedBaggageData[]> | null | undefined
  >(undefined);
  const [startQpidId, setStartQpidArea] = useState<number>(defaultQpidId ?? -1);
  const [destinationQpidId, setDestinationQpidArea] = useState<number>(-1);

  useEffect(() => {
    setLoading(true);

    (async () => {
      const [qpidAreas, lostBaggages] = await Promise.all([
        getQpidAreas(),
        getLostBaggages("en-us"),
      ]);

      setLoading(false);

      if (qpidAreas.data != null && lostBaggages.data != null) {
        setQpidAreas(
          qpidAreas.data
            .filter(
              (a) =>
                a.metadata.area === area &&
                allowedConstructionTypes.includes(
                  a.metadata.constructionType
                ) &&
                lostBaggages.data[a.qpidId] != null
            )
            .sort((a, b) => a.names["en-us"].localeCompare(b.names["en-us"]))
        );
      } else {
        setQpidAreas(null);
        alert(`Failed to load qpid areas: ${qpidAreas.error}`);
      }

      if (lostBaggages.data != null) {
        setLostBaggages(lostBaggages.data);
      } else {
        setLostBaggages(null);
        alert(`Failed to load lost baggages: ${lostBaggages.error}`);
      }
    })();
  }, []);

  return !loading && qpidAreas != null && lostBaggages != null ? (
    <Form.Root>
      <Form.Field>
        <Form.Label>Start</Form.Label>
        <Form.Select
          value={startQpidId}
          onChange={(e) => setStartQpidArea(e.target.value)}
        >
          <option value={-1}>Select Start</option>
          {qpidAreas.map((qpidArea) => (
            <option key={qpidArea.qpidId} value={qpidArea.qpidId}>
              {qpidArea.names["en-us"]}
            </option>
          ))}
        </Form.Select>
      </Form.Field>
      <Form.Field>
        <Form.Label>Destination</Form.Label>
        <Form.Select
          value={destinationQpidId}
          onChange={(e) => setDestinationQpidArea(e.target.value)}
        >
          <option value={-1}>Select Destination</option>
          {qpidAreas.map((qpidArea) => (
            <option key={qpidArea.qpidId} value={qpidArea.qpidId}>
              {qpidArea.names["en-us"]}
            </option>
          ))}
        </Form.Select>
      </Form.Field>
      <Form.Field>
        <Form.Label>Baggage</Form.Label>
        <Form.Select disabled={destinationQpidId === -1} value={-1}>
          <option value={-1}>Select Baggage</option>
          {destinationQpidId !== -1 &&
            lostBaggages[destinationQpidId].map((baggage) => (
              <option key={baggage.nameHash} value={baggage.nameHash}>
                {baggage.name} ({baggage.baggageMetadata.typeVolume},{" "}
                {baggage.baggageMetadata.weight} kg)
              </option>
            ))}
        </Form.Select>
      </Form.Field>
    </Form.Root>
  ) : (
    "Loading data..."
  );
};

export default MissionEditor;
