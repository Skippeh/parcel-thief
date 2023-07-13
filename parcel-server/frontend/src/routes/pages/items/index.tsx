import * as React from "react";
import ContentBox from "../../layout/content_box";
import PageTitle from "../../../components/page_title";
import Table from "./table";
import { useState } from "react";
import { getSharedCargo } from "../../../services/baggages_service";

const Items = () => {
  const [items, setItems] = useState(undefined);

  React.useEffect(() => {
    // fetch items
    (async () => {
      const baggages = await getSharedCargo();
    })();
  }, []);

  return (
    <div>
      <PageTitle>Items</PageTitle>
      <ContentBox>
        <Table />
      </ContentBox>
    </div>
  );
};

export default Items;
