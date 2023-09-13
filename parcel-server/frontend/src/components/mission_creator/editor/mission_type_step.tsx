import { EditMissionData } from "../../../api_types";
import * as Form from "../../form";

interface Props {
  data: EditMissionData | null;
  setData: (data: EditMissionData | null) => void;
  defaultQpidId?: number;
}

const MissionTypeStep = ({ data, setData, defaultQpidId }: Props) => {
  function onMissionTypeChanged(event: React.ChangeEvent<HTMLSelectElement>) {
    if (
      data == null ||
      confirm(
        "Warning: Changing mission type will reset all mission data. Are you sure?"
      )
    ) {
      const newValue = event.target.value;

      if (newValue === "delivery") {
        setData({
          type: newValue,
          startQpidId: defaultQpidId ?? -1,
          endQpidId: -1,
          baggageAmounts: [],
        });
      } else if (newValue === "collection") {
        setData({
          type: newValue,
          targetQpidId: defaultQpidId ?? -1,
          baggageAmounts: [],
        });
      } else if (newValue === "recovery") {
        setData({
          type: newValue,
          targetQpidId: defaultQpidId ?? -1,
          baggages: [],
        });
      } else if (newValue === "") {
        setData(null);
      } else {
        throw new Error(`Unknown mission type: ${newValue}`);
      }
    }
  }

  return (
    <div>
      <Form.Root>
        <Form.Field>
          <Form.Label>Mission Type</Form.Label>
          <Form.Select value={data?.type ?? ""} onChange={onMissionTypeChanged}>
            <option value="">Select mission type</option>
            <option value="delivery">Delivery</option>
            <option value="collection">Collection</option>
            <option value="recovery">Recovery</option>
          </Form.Select>
        </Form.Field>
      </Form.Root>
      <p>
        Note that all mission types require the player to have the relevant
        areas synced up with the 'chiral network' in the game.
      </p>
      <h3>Mission descriptions</h3>
      <ul>
        <li>
          <strong>Delivery</strong>
          <p>
            Pick up cargo from a shared cargo box and deliver it to another.
          </p>
        </li>
        <li>
          <strong>Collection</strong>
          <p>
            Deliver a specified amount of resources or tools to a shared cargo
            box.
          </p>
        </li>
        <li>
          <strong>Recovery</strong>
          <p>
            Recover specific lost cargo from the world and delivery it to a
            shared cargo box.
          </p>
          <p>
            <span className="warning">
              Note: This mission type is experimental and might not work
              properly due to the game potentially limiting the amount of lost
              cargo being spawned at once.
            </span>
          </p>
        </li>
      </ul>
    </div>
  );
};

export default MissionTypeStep;
