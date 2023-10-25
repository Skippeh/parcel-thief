import {
  EditMissionData,
  LocalizedBaggageData,
  QpidArea,
} from "../../../../api_types";
import * as Form from "../../../form";
import BaggageSelector from "../../baggage_selector";
import LocationSelector from "../../location_selector";
import CargoAmountSelector, {
  SelectedCargo,
} from "../../cargo_amount_selector";
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
  qpidAreas: Record<number, QpidArea>,
  lostBaggages: Record<number, LocalizedBaggageData[]>,
  rewardBaggages: LocalizedBaggageData[]
) {
  const locations = Object.values(qpidAreas);
  const flatLostBaggages = Object.values(lostBaggages).flat();
  const selectedCargo: SelectedCargo[] = data.baggageAmounts.map(
    ({ nameHash, amount }) => {
      const baggage = flatLostBaggages.find((b) => b.nameHash === nameHash);

      if (baggage == null) {
        throw new Error(`Baggage not found: ${nameHash}`);
      }

      return {
        cargo: baggage,
        amount,
      };
    }
  );

  const selectedRewards: SelectedCargo[] = data.rewardAmounts.map(
    ({ nameHash, amount }) => {
      const baggage = rewardBaggages.find((b) => b.nameHash === nameHash);

      if (baggage == null) {
        throw new Error(`Baggage not found: ${nameHash}`);
      }

      return {
        cargo: baggage,
        amount,
      };
    }
  );

  function onCargoChanged(values: SelectedCargo[]) {
    const baggageAmounts = values.map(({ cargo, amount }) => ({
      nameHash: cargo.nameHash,
      amount,
    }));

    setData({
      ...data,
      baggageAmounts,
    });
  }

  function onRewardChanged(values: SelectedCargo[]) {
    const baggageAmounts = values.map(({ cargo, amount }) => ({
      nameHash: cargo.nameHash,
      amount,
    }));

    setData({
      ...data,
      rewardAmounts: baggageAmounts,
    });
  }

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
      <div>
        {data.endQpidId > 0 && lostBaggages[data.endQpidId] && (
          <Form.Field>
            <Form.Label>Cargo</Form.Label>
            <CargoAmountSelector
              values={selectedCargo}
              onChange={onCargoChanged}
              baggages={lostBaggages[data.endQpidId] ?? []}
            />
          </Form.Field>
        )}
      </div>
      <div>
        <Form.Field>
          <Form.Label>Reward</Form.Label>
          <CargoAmountSelector
            values={selectedRewards}
            onChange={onRewardChanged}
            baggages={rewardBaggages}
          />
        </Form.Field>
      </div>
    </>
  );
}
