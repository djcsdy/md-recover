use crate::block_device::BlockDevice;
use crate::lvm::metadata::{Metadata, MetadataArea};
use crate::lvm::PhysicalVolume;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Seek};

#[derive(Clone)]
pub struct VolumeGroup<D>
where
    D: BlockDevice + Read + Seek,
{
    metadata: Metadata,
    physical_volumes: Vec<PhysicalVolume<D>>,
}

impl<D> VolumeGroup<D>
where
    D: BlockDevice + Read + Seek,
{
    pub fn open(physical_volumes: Vec<PhysicalVolume<D>>) -> io::Result<Self> {
        let (metadata_by_uuid, errors) = physical_volumes
            .iter()
            .flat_map(PhysicalVolume::metadata_areas)
            .flat_map(MetadataArea::metadata)
            .map(Result::as_ref)
            .map(|metadata| {
                metadata.map_err(Into::into).and_then(|metadata| {
                    metadata.uuid().and_then(|uuid| {
                        metadata
                            .sequence_number()
                            .map(|sequence_number| (uuid, sequence_number, metadata))
                    })
                })
            })
            .fold(
                (HashMap::new(), Vec::new()),
                |(mut metadata_by_uuid, mut errors), result| {
                    match result {
                        Ok((uuid, sequence_number, metadata)) => {
                            match metadata_by_uuid.entry(uuid.to_owned()) {
                                Entry::Occupied(mut entry) => {
                                    let (existing_sequence_number, existing_metadata) =
                                        entry.get_mut();
                                    if *existing_sequence_number < sequence_number {
                                        *existing_sequence_number = sequence_number;
                                        *existing_metadata = metadata.clone();
                                    }
                                }
                                Entry::Vacant(entry) => {
                                    entry.insert((sequence_number, metadata.clone()));
                                }
                            }
                        }
                        Err(err) => errors.push(err.clone()),
                    }

                    (metadata_by_uuid, errors)
                },
            );

        let metadata = metadata_by_uuid
            .into_values()
            .exactly_one()
            .map(|(_, metadata)| metadata)
            .map_err(|_| {
                errors
                    .into_iter()
                    .next()
                    .map(|error| io::Error::new(io::ErrorKind::InvalidData, error))
                    .unwrap_or_else(|| io::ErrorKind::InvalidData.into())
            })?;

        Ok(Self {
            metadata,
            physical_volumes,
        })
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        self.physical_volumes
            .iter()
            .map(|physical_volume| physical_volume.try_clone())
            .try_collect()
            .map(|physical_volumes| Self {
                metadata: self.metadata.clone(),
                physical_volumes,
            })
    }
}
