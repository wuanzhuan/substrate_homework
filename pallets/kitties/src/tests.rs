use crate::{Error, mock::*, pallet};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_create() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

        crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id)),
            Error::<Test>::InvalidKittyId
        );
    })
}

#[test]
fn it_works_for_breed() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
            Error::<Test>::SameKittyId
        );
        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
            Error::<Test>::InvalidKittyId
        );

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

        assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1));

        let breed_kitty_id = 2;
        assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
        assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));

    })
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let recipient = 2;

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), recipient, kitty_id), Error::<Test>::NotOwner);
        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));

        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id));

        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
    })
}

#[test]
fn check_events_for_create() {
    use frame_system::pallet_prelude::*;

    new_test_ext().execute_with(|| {

        let kitty_id = KittiesModule::next_kitty_id();
        let account_id = 1;

        let origin = RuntimeOrigin::signed(account_id);
        let who = ensure_signed(origin.clone());
        assert_ok!(&who);

        assert_ok!(KittiesModule::create(origin));

        println!("kitty_id: {}", kitty_id);
        let kitty = KittiesModule::kitties(kitty_id);
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);

        System::assert_has_event(pallet::Event::<Test>::KittyCreated { who: who.unwrap(), kitty_id, kitty: kitty.unwrap()}.into());

    })
}

#[test]
fn check_events_for_breed() {
    use frame_system::pallet_prelude::*;

    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        let origin = RuntimeOrigin::signed(account_id);
        let who = ensure_signed(origin.clone());
        assert_ok!(&who);

        assert_ok!(KittiesModule::create(origin.clone()));
        assert_ok!(KittiesModule::create(origin.clone()));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

        assert_ok!(KittiesModule::breed(origin.clone(), kitty_id, kitty_id + 1));

        let kitty = KittiesModule::kitties(kitty_id + 2);
        assert_eq!(kitty.is_some(), true);

        System::assert_has_event(pallet::Event::<Test>::KittyBreed{ who: who.unwrap(), kitty_id: kitty_id + 2, kitty: kitty.unwrap()}.into());

    })
}

#[test]
fn check_events_for_transfer() {
    use frame_system::pallet_prelude::*;

    new_test_ext().execute_with(|| {

        let kitty_id = 0;
        let account_id = 1;
        let recipient = 2;

        let origin = RuntimeOrigin::signed(account_id);
        let who = ensure_signed(origin.clone());
        assert_ok!(&who);

        assert_ok!(KittiesModule::create(origin.clone()));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));

        System::assert_has_event(pallet::Event::<Test>::KittyTransferred { who: who.unwrap(), receipt: recipient, kitty_id }.into());

    })
}

