import { Option, Vec, BTreeMap, u64, bool, Text, Null, Bytes } from '@polkadot/types'
import { BlockAndTime, JoyEnum, JoyStructDecorated, Hash, ChannelId, DAOId, WorkingGroup } from './common'
import { MemberId } from './members'
import { StorageProviderId } from './working-group' // this should be in discovery really
import { randomAsU8a } from '@polkadot/util-crypto'
import { encodeAddress, decodeAddress } from '@polkadot/keyring'
import { RegistryTypes, Registry } from '@polkadot/types/types'

export class ContentId extends Hash {
  static generate(registry: Registry): ContentId {
    // randomAsU8a uses https://www.npmjs.com/package/tweetnacl#random-bytes-generation
    return new ContentId(registry, randomAsU8a())
  }

  static decode(registry: Registry, contentId: string): ContentId {
    return new ContentId(registry, decodeAddress(contentId))
  }

  static encode(contentId: Uint8Array): string {
    // console.log('contentId:', Buffer.from(contentId).toString('hex'))
    return encodeAddress(contentId)
  }

  encode(): string {
    return ContentId.encode(this)
  }
}

export class DataObjectTypeId extends u64 {}
export class DataObjectStorageRelationshipId extends u64 {}

export class VecContentId extends Vec.with(ContentId) {}
export class OptionVecContentId extends Option.with(VecContentId) {}

export const LiaisonJudgementDef = {
  Pending: Null,
  Accepted: Null,
  Rejected: Null,
} as const
export type LiaisonJudgementKey = keyof typeof LiaisonJudgementDef
export class LiaisonJudgement extends JoyEnum(LiaisonJudgementDef) {}

export class StorageObjectOwner extends JoyEnum({
  Member: MemberId,
  Channel: ChannelId,
  DAO: DAOId,
  Council: Null,
  WorkingGroup: WorkingGroup,
}) {}

export class ContentParameters extends JoyStructDecorated({
  content_id: ContentId,
  type_id: DataObjectTypeId,
  size: u64,
  ipfs_content_id: Bytes,
}) {
  /** Actually it's 'size', but 'size' is already reserved by a parent class. */
  get size_in_bytes(): u64 {
    return this.get('size') as u64
  }
}

export class Content extends Vec.with(ContentParameters) {}

export class DataObject extends JoyStructDecorated({
  owner: StorageObjectOwner,
  added_at: BlockAndTime,
  type_id: DataObjectTypeId,
  size: u64,
  liaison: StorageProviderId,
  liaison_judgement: LiaisonJudgement,
  ipfs_content_id: Text,
}) {
  /** Actually it's 'size', but 'size' is already reserved by a parent class. */
  get size_in_bytes(): u64 {
    return this.get('size') as u64
  }
}

export class DataObjectStorageRelationship extends JoyStructDecorated({
  content_id: ContentId,
  storage_provider: StorageProviderId,
  ready: bool,
}) {}

export class DataObjectType extends JoyStructDecorated({
  description: Text,
  active: bool,
}) {}

export class DataObjectsMap extends BTreeMap.with(ContentId, DataObject) {}

export class Quota extends JoyStructDecorated({
  // Total objects size limit per StorageObjectOwner
  size_limit: u64,
  // Total objects number limit per StorageObjectOwner
  objects_limit: u64,
  size_used: u64,
  objects_used: u64,
}) {}

export class QuotaLimit extends u64 {}
export class UploadingStatus extends bool {}

export const mediaTypes: RegistryTypes = {
  ContentId,
  LiaisonJudgement,
  DataObject,
  DataObjectStorageRelationshipId,
  DataObjectStorageRelationship,
  DataObjectTypeId,
  DataObjectType,
  DataObjectsMap,
  ContentParameters,
  StorageObjectOwner,
  Content,
  Quota,
  QuotaLimit,
  UploadingStatus,
}

export default mediaTypes
