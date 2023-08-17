import * as React from "react";
import SharedCargoTable from "./shared_cargo_table";
import { useState } from "react";
import {
  getLostCargoList,
  getSharedCargoList,
  getWastedCargoList,
} from "../../../services/baggages_service";
import {
  LostCargoListItem,
  SharedCargoListItem,
  WastedCargoListItem,
} from "../../../api_types";
import * as Tabs from "../../../components/tabs";
import LostCargoTable from "./lost_cargo_table";
import WastedCargoTable from "./wasted_cargo_table";

const Items = () => {
  const [sharedItems, setSharedItems] = useState<
    SharedCargoListItem[] | undefined | null
  >();

  const [lostItems, setLostItems] = useState<
    LostCargoListItem[] | undefined | null
  >();

  const [wastedItems, setWastedItems] = useState<
    WastedCargoListItem[] | undefined | null
  >();

  React.useEffect(() => {
    // fetch items if there are no items yet
    (async () => {
      if (sharedItems == null) {
        const response = await getSharedCargoList();

        if (response.data != null) {
          setSharedItems(response.data.baggages);
        } else {
          console.error(response.statusCode, response.error);
          setSharedItems(null);
        }
      }

      if (lostItems == null) {
        const response = await getLostCargoList();

        if (response.data != null) {
          setLostItems(response.data.baggages);
        } else {
          console.error(response.statusCode, response.error);
          setLostItems(null);
        }
      }

      if (wastedItems == null) {
        const response = await getWastedCargoList();

        if (response.data != null) {
          setWastedItems(response.data.baggages);
        } else {
          console.error(response.statusCode, response.error);
          setWastedItems(null);
        }
      }
    })();
  }, []);

  return (
    <>
      <Tabs.Root defaultValue="sharedCargo">
        <Tabs.List>
          <Tabs.Trigger value="sharedCargo">Shared cargo</Tabs.Trigger>
          <Tabs.Trigger value="lostCargo">Lost cargo</Tabs.Trigger>
          <Tabs.Trigger value="wastedCargo">Wasted cargo</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="sharedCargo" forceMount>
          <SharedCargoTable items={sharedItems} />
        </Tabs.Content>
        <Tabs.Content value="lostCargo" forceMount>
          <LostCargoTable items={lostItems} />
        </Tabs.Content>
        <Tabs.Content value="wastedCargo" forceMount>
          <WastedCargoTable items={wastedItems} />
        </Tabs.Content>
      </Tabs.Root>
    </>
  );
};

export default Items;
