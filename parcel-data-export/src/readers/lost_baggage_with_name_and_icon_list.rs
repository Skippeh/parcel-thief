use parcel_game_data::Area;

use super::{
    array::Array, baggage_list_item::BaggageListItem,
    delivery_point_info_resource::DeliveryPointInfoResource, reference::Ref, resource::Resource,
    RTTITypeHash,
};

#[derive(Debug, Clone)]
pub struct LostBaggageWithNameAndIconListResource {
    base: Resource,
    pub destination: Ref<DeliveryPointInfoResource>,
    pub baggages: Array<Ref<BaggageListItem>>,
}

impl super::Read for LostBaggageWithNameAndIconListResource {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = Resource::read(reader, context)?;
        let destination = Ref::read(reader, context)?;
        let baggages = Array::read(reader, context)?;

        Ok(LostBaggageWithNameAndIconListResource {
            base,
            destination,
            baggages,
        })
    }
}

impl super::ReadRTTIType for LostBaggageWithNameAndIconListResource {
    fn rtti_type() -> RTTITypeHash {
        RTTITypeHash::LostBaggageWithNameAndIconListResource
    }
}

impl std::ops::Deref for LostBaggageWithNameAndIconListResource {
    type Target = Resource;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LostBaggageWithNameAndIconListResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Clone)]
pub struct LostBaggageWithNameAndIconListCollection {
    base: Resource,
    pub area: Area,
    pub list: Array<Ref<LostBaggageWithNameAndIconListResource>>,
}

impl super::Read for LostBaggageWithNameAndIconListCollection {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = Resource::read(reader, context)?;
        let area = Area::read(reader, context)?;
        let list = Array::read(reader, context)?;

        Ok(LostBaggageWithNameAndIconListCollection { base, area, list })
    }
}

impl super::ReadRTTIType for LostBaggageWithNameAndIconListCollection {
    fn rtti_type() -> RTTITypeHash {
        RTTITypeHash::LostBaggageWithNameAndIconListCollection
    }
}

impl std::ops::Deref for LostBaggageWithNameAndIconListCollection {
    type Target = Resource;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LostBaggageWithNameAndIconListCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
