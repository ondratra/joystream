#![cfg(test)]

use common::storage::StorageObjectOwner;
use frame_support::dispatch::DispatchError;
use sp_std::collections::btree_map::BTreeMap;
use system::RawOrigin;

use super::mock::*;

#[test]
fn succeed_adding_content() {
    with_default_mock_builder(|| {
        let sender = 1u64;
        let owner = StorageObjectOwner::Member(1u64);

        let first_content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: 0,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        let second_content_parameters = ContentParameters {
            content_id: 2,
            type_id: 2,
            size: 20,
            ipfs_content_id: vec![1, 2, 7, 9],
        };

        let multi_content = vec![first_content_parameters, second_content_parameters];

        // Register a content with 1234 bytes of type 1, which should be recognized.
        let res = TestDataDirectory::add_content(Origin::signed(sender), owner, multi_content);
        assert!(res.is_ok());
    });
}

#[test]
fn add_content_fails_with_invalid_origin() {
    with_default_mock_builder(|| {
        let owner = StorageObjectOwner::Member(1u64);

        let content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: 0,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        // Register a content with 1234 bytes of type 1, which should be recognized.
        let res =
            TestDataDirectory::add_content(RawOrigin::Root.into(), owner, vec![content_parameters]);
        assert_eq!(res, Err(DispatchError::Other("Bad origin")));
    });
}

#[test]
fn accept_and_reject_content_fail_with_invalid_storage_provider() {
    with_default_mock_builder(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let sender = 1u64;
        let owner = StorageObjectOwner::Member(1u64);

        let content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: 0,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        let res =
            TestDataDirectory::add_content(Origin::signed(sender), owner, vec![content_parameters]);
        assert!(res.is_ok());

        let content_id = match &System::events().last().unwrap().event {
            MetaEvent::data_directory(data_directory::RawEvent::ContentAdded(content, _)) => {
                content[0].content_id
            }
            _ => 0u64,
        };

        //  invalid data
        let (storage_provider_account_id, storage_provider_id) = (1, 5);

        let res = TestDataDirectory::accept_content(
            Origin::signed(storage_provider_account_id),
            storage_provider_id,
            content_id,
        );
        assert_eq!(res, Err(working_group::Error::<Test, crate::StorageWorkingGroupInstance>::WorkerDoesNotExist.into()));

        let res = TestDataDirectory::reject_content(
            Origin::signed(storage_provider_account_id),
            storage_provider_id,
            content_id,
        );
        assert_eq!(res, Err(working_group::Error::<Test, crate::StorageWorkingGroupInstance>::WorkerDoesNotExist.into()));
    });
}

#[test]
fn accept_content_as_liaison() {
    with_default_mock_builder(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let sender = 1u64;
        let owner = StorageObjectOwner::Member(1u64);

        let content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: 0,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        let res =
            TestDataDirectory::add_content(Origin::signed(sender), owner, vec![content_parameters]);
        assert!(res.is_ok());

        // An appropriate event should have been fired.
        let (content_id, creator) = match &System::events().last().unwrap().event {
            MetaEvent::data_directory(data_directory::RawEvent::ContentAdded(content, creator)) => {
                (content[0].content_id, creator.clone())
            }
            _ => (0u64, StorageObjectOwner::Member(0xdeadbeefu64)), // invalid value, unlikely to match
        };
        assert_ne!(creator, StorageObjectOwner::Member(0xdeadbeefu64));
        assert_eq!(creator, StorageObjectOwner::Member(sender));

        let (storage_provider_account_id, storage_provider_id) = hire_storage_provider();

        // Accepting content should not work with some random origin
        let res =
            TestDataDirectory::accept_content(Origin::signed(55), storage_provider_id, content_id);
        assert!(res.is_err());

        // However, with the liaison as origin it should.
        let res = TestDataDirectory::accept_content(
            Origin::signed(storage_provider_account_id),
            storage_provider_id,
            content_id,
        );
        assert_eq!(res, Ok(()));
    });
}

#[test]
fn reject_content_as_liaison() {
    with_default_mock_builder(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let sender = 1u64;
        let owner = StorageObjectOwner::Member(1u64);

        let content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: 0,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        let res =
            TestDataDirectory::add_content(Origin::signed(sender), owner, vec![content_parameters]);
        assert!(res.is_ok());

        // An appropriate event should have been fired.
        let (content_id, creator) = match &System::events().last().unwrap().event {
            MetaEvent::data_directory(data_directory::RawEvent::ContentAdded(content, creator)) => {
                (content[0].content_id, creator.clone())
            }
            _ => (0u64, StorageObjectOwner::Member(0xdeadbeefu64)), // invalid value, unlikely to match
        };
        assert_ne!(creator, StorageObjectOwner::Member(0xdeadbeefu64));
        assert_eq!(creator, StorageObjectOwner::Member(sender));

        let (storage_provider_account_id, storage_provider_id) = hire_storage_provider();

        // Rejecting content should not work with some random origin
        let res =
            TestDataDirectory::reject_content(Origin::signed(55), storage_provider_id, content_id);
        assert!(res.is_err());

        // However, with the liaison as origin it should.
        let res = TestDataDirectory::reject_content(
            Origin::signed(storage_provider_account_id),
            storage_provider_id,
            content_id,
        );
        assert_eq!(res, Ok(()));
    });
}

#[test]
fn data_object_injection_works() {
    with_default_mock_builder(|| {
        // No objects in directory before injection
        assert_eq!(TestDataDirectory::known_content_ids(), vec![]);

        // new objects to inject into the directory
        let mut objects = BTreeMap::new();

        let object = data_directory::DataObjectInternal {
            type_id: 1,
            size: 1234,
            added_at: data_directory::BlockAndTime {
                block: 10,
                time: 1024,
            },
            owner: StorageObjectOwner::Member(1),
            liaison: TEST_MOCK_LIAISON_STORAGE_PROVIDER_ID,
            liaison_judgement: data_directory::LiaisonJudgement::Pending,
            ipfs_content_id: vec![],
        };

        let content_id_1 = 1;
        objects.insert(content_id_1, object.clone());

        let content_id_2 = 2;
        objects.insert(content_id_2, object.clone());

        let res = TestDataDirectory::inject_data_objects(RawOrigin::Root.into(), objects);
        assert!(res.is_ok());

        assert_eq!(
            TestDataDirectory::known_content_ids(),
            vec![content_id_1, content_id_2]
        );

        assert_eq!(
            TestDataDirectory::data_object_by_content_id(content_id_1),
            Some(object.clone())
        );

        assert_eq!(
            TestDataDirectory::data_object_by_content_id(content_id_2),
            Some(object)
        );
    });
}

#[test]
fn data_object_injection_overwrites_and_removes_duplicate_ids() {
    with_default_mock_builder(|| {
        let sender = 1u64;
        let owner = StorageObjectOwner::Member(1u64);
        let content_id_1 = 1;
        let content_id_2 = 2;

        let content_parameters_first = ContentParameters {
            content_id: content_id_1,
            type_id: 1,
            size: 10,
            ipfs_content_id: vec![8, 8, 8, 8],
        };

        let content_parameters_second = ContentParameters {
            content_id: content_id_2,
            type_id: 2,
            size: 20,
            ipfs_content_id: vec![9, 9, 9, 9],
        };

        // Start with some existing objects in directory which will be
        // overwritten
        let res = TestDataDirectory::add_content(
            Origin::signed(sender),
            owner.clone(),
            vec![content_parameters_first],
        );
        assert!(res.is_ok());
        let res = TestDataDirectory::add_content(
            Origin::signed(sender),
            owner,
            vec![content_parameters_second],
        );
        assert!(res.is_ok());

        let mut objects = BTreeMap::new();

        let object1 = data_directory::DataObjectInternal {
            type_id: 1,
            size: 6666,
            added_at: data_directory::BlockAndTime {
                block: 10,
                time: 1000,
            },
            owner: StorageObjectOwner::Member(5),
            liaison: TEST_MOCK_LIAISON_STORAGE_PROVIDER_ID,
            liaison_judgement: data_directory::LiaisonJudgement::Pending,
            ipfs_content_id: vec![5, 6, 7],
        };

        let object2 = data_directory::DataObjectInternal {
            type_id: 1,
            size: 7777,
            added_at: data_directory::BlockAndTime {
                block: 20,
                time: 2000,
            },
            owner: StorageObjectOwner::Member(6),
            liaison: TEST_MOCK_LIAISON_STORAGE_PROVIDER_ID,
            liaison_judgement: data_directory::LiaisonJudgement::Pending,
            ipfs_content_id: vec![5, 6, 7],
        };

        objects.insert(content_id_1, object1.clone());
        objects.insert(content_id_2, object2.clone());

        let res = TestDataDirectory::inject_data_objects(RawOrigin::Root.into(), objects);
        assert!(res.is_ok());

        assert_eq!(
            TestDataDirectory::known_content_ids(),
            vec![content_id_1, content_id_2]
        );

        assert_eq!(
            TestDataDirectory::data_object_by_content_id(content_id_1),
            Some(object1.clone())
        );

        assert_eq!(
            TestDataDirectory::data_object_by_content_id(content_id_2),
            Some(object2)
        );
    });
}
