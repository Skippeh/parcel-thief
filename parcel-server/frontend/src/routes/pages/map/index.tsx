import { useParams } from "react-router-dom";
import styled from "styled-components";
import GameMap from "../../../components/game_map";
import { Area } from "../../../api_types";

const Wrapper = styled.div`
  height: 100%;
`;

const Map = () => {
  const { area } = useParams();

  function mapArea(areaParam: string): Area {
    switch (areaParam) {
      case "east":
        return "area01";
      case "central":
        return "area02";
      case "west":
        return "area04";
    }
  }

  return (
    <Wrapper>
      <GameMap area={mapArea(area)} />
    </Wrapper>
  );
};

export default Map;
