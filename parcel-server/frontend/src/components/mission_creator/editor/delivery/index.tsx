import { EditMissionData, QpidArea } from "../../../../api_types";
import * as Form from "../../../form";
import LocationSelector from "../../location_selector";
import { Step } from "../header";

export function renderDeliveryHeaderSteps(
  data: (EditMissionData & { type: "delivery" }) | null
) {
  return (
    <>
      <Step step={1}>Pickup location</Step>
      <Step step={2}>Dropoff location</Step>
      <Step
        step={3}
        disabled={data.endQpidId <= 0}
        title={(data.endQpidId <= 0 && "Choose dropoff location first") || ""}
      >
        Cargo
      </Step>
      <Step step={4}>Reward</Step>
    </>
  );
}

export function renderDeliverySteps(
  data: EditMissionData & { type: "delivery" },
  setData: (data: EditMissionData | null) => void,
  qpidAreas: Record<number, QpidArea>
) {
  const locations = Object.values(qpidAreas);

  return (
    <>
      <div>
        <Form.Field>
          <Form.Label>Pickup location</Form.Label>
          <LocationSelector
            locations={locations}
            value={qpidAreas[data.startQpidId]}
            onChange={(qpidArea) =>
              setData({ ...data, startQpidId: qpidArea?.qpidId ?? 0 })
            }
          />
        </Form.Field>
      </div>
      <div>
        <Form.Field>
          <Form.Label>Dropoff location</Form.Label>
          <LocationSelector
            locations={locations}
            referenceLocation={qpidAreas[data.startQpidId]}
            value={qpidAreas[data.endQpidId]}
            onChange={(qpidArea) =>
              setData({ ...data, endQpidId: qpidArea?.qpidId ?? 0 })
            }
          />
        </Form.Field>
      </div>
      <div>cargo</div>
      <div>reward</div>
    </>
  );
}
