import { useEffect, useState } from "react";
import {
  Area,
  BaggageAmount,
  ConstructionPointType,
  EditMissionData,
  LocalizedBaggageData,
  QpidArea,
} from "../../../api_types";
import {
  getLostBaggages,
  getQpidAreas,
} from "../../../services/game_data_service";
import { Wizard } from "../../wizard";
import Header from "./header";
import MissionTypeStep from "./mission_type_step";
import styled from "styled-components";
import * as Form from "../../form";
import { renderDeliverySteps } from "./delivery";

const Wrapper = styled.div`
  display: grid;
  grid-template-columns: 250px 1fr;
  max-height: 100vh;
  height: 500px;
  gap: 1.5rem;
`;

const StepsWrapper = styled.div`
  overflow: auto;
`;

const allowedConstructionTypes: ConstructionPointType[] = [
  "deliveryBase",
  "preppersShelter",
];

interface Props {
  area: Area;
  startQpidId?: number;
}

const MissionEditor = ({ area, startQpidId: defaultQpidId }: Props) => {
  const [loading, setLoading] = useState(false);
  const [qpidAreas, setQpidAreas] = useState<
    Record<number, QpidArea> | null | undefined
  >(undefined);
  const [lostBaggages, setLostBaggages] = useState<
    Record<number, LocalizedBaggageData[]> | null | undefined
  >(undefined);
  const [data, setData] = useState<EditMissionData | null>(null);

  useEffect(() => {
    setLoading(true);
    setQpidAreas(undefined);
    setLostBaggages(undefined);

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
            .reduce((acc, qpidArea) => {
              acc[qpidArea.qpidId] = qpidArea;
              return acc;
            }, {})
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
      <Wrapper>
        <Wizard header={<Header data={data} />} wrapper={<StepsWrapper />}>
          <MissionTypeStep
            data={data}
            setData={setData}
            defaultQpidId={defaultQpidId}
          />
          {data?.type === "delivery" &&
            renderDeliverySteps(data, setData, qpidAreas)}
        </Wizard>
      </Wrapper>
    </Form.Root>
  ) : (
    "Loading data..."
  );
};

export default MissionEditor;
