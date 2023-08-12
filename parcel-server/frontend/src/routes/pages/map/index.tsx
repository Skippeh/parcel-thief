import { useParams } from "react-router-dom";
import styled from "styled-components";
import GameMap, { Area } from "../../../components/game_map";

const Wrapper = styled.div`
  height: 100%;
`;

const Map = () => {
  const { area } = useParams();

  function mapArea(areaParam: string): Area {
    switch (areaParam) {
      case "east":
        return Area.East;
      case "central":
        return Area.Central;
      case "west":
        return Area.West;
    }
  }

  return (
    <Wrapper>
      <GameMap area={mapArea(area)} />
    </Wrapper>
  );
};

export default Map;
