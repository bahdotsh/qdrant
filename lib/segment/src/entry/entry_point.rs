use thiserror::Error;
use std::path::Path;
use crate::types::{SeqNumberType, VectorElementType, Filter, PointIdType, PayloadKeyType, PayloadType, SearchParams, ScoredPoint, TheMap, SegmentInfo};
use std::result;
use sled::Error;


/// Trait for versionable & saveable objects.
pub trait VersionedPersistable {
    fn persist(&self, directory: &Path) -> SeqNumberType;
    fn load(directory: &Path) -> Self;

    /// Save latest persisted version in memory, so the object will not be saved too much times
    fn ack_persistance(&mut self, version: SeqNumberType);
}


#[derive(Error, Debug)]
#[error("{0}")]
pub enum OperationError {
    #[error("Vector inserting error: expected dim: {expected_dim}, got {received_dim}")]
    WrongVector { expected_dim: usize, received_dim: usize },
    #[error("No point with id {missed_point_id} found")]
    PointIdError { missed_point_id: PointIdType },
    #[error("Service runtime error: {description}")]
    ServiceError { description: String }
}


impl From<Error> for OperationError {
    fn from(err: Error) -> Self {
        OperationError::ServiceError { description: format!("persistence error: {:?}", err) }
    }
}

pub type Result<T> = result::Result<T, OperationError>;

pub type OperationResult<T> = result::Result<T, OperationError>;


/// Define all operations which can be performed with Segment.
/// Assume, that all operations are idempotent - which means that
///     no matter how much time they will consequently executed - storage state will be the same.
pub trait SegmentEntry {
    /// Get current update version of the segment
    fn version(&self) -> SeqNumberType;

    fn search(&self,
              vector: &Vec<VectorElementType>,
              filter: Option<&Filter>,
              top: usize,
              params: Option<&SearchParams>,
    ) -> Result<Vec<ScoredPoint>>;

    fn upsert_point(&mut self, op_num: SeqNumberType, point_id: PointIdType, vector: &Vec<VectorElementType>) -> Result<bool>;

    fn delete_point(&mut self, op_num: SeqNumberType, point_id: PointIdType) -> Result<bool>;

    fn set_full_payload(&mut self, op_num: SeqNumberType, point_id: PointIdType, full_payload: TheMap<PayloadKeyType, PayloadType>)-> Result<bool>;

    fn set_payload(&mut self, op_num: SeqNumberType, point_id: PointIdType, key: &PayloadKeyType, payload: PayloadType) -> Result<bool>;

    fn delete_payload(&mut self, op_num: SeqNumberType, point_id: PointIdType, key: &PayloadKeyType) -> Result<bool>;

    fn clear_payload(&mut self, op_num: SeqNumberType, point_id: PointIdType) -> Result<bool>;

    fn vector(&self, point_id: PointIdType) -> Result<Vec<VectorElementType>>;

    fn payload(&self, point_id: PointIdType) -> Result<TheMap<PayloadKeyType, PayloadType>>;

    fn iter_points(&self) -> Box<dyn Iterator<Item=PointIdType> + '_>;

    /// Check if there is point with `point_id` in this segment.
    fn has_point(&self, point_id: PointIdType) -> bool;

    /// Return number of vectors in this segment
    fn vectors_count(&self) -> usize;

    fn info(&self) -> SegmentInfo;

}

