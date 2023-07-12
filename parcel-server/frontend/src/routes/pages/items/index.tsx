import * as React from "react";
import ContentBox from "../../layout/content_box";
import PageTitle from "../../../components/page_title";
import Table from "./table";

const Items = () => {
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
