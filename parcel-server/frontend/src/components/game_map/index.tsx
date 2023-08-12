import { styled } from "styled-components";

export enum Area {
  East = "area01",
  Central = "area02",
  West = "area04",
}

const Wrapper = styled.div`
  height: 100%;
`;

interface Props {
  area: Area;
}

const GameMap = ({ area }: Props) => {
  return <Wrapper>GameMap of {area}</Wrapper>;
};

export default GameMap;
