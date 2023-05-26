use super::*;
use crate::{mock::*, Error};
use frame_support::traits::{ConstU32};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::<u8, ConstU32<10>>::try_from(vec![0, 1]).unwrap();

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone().to_vec()));
        assert_eq!(Proofs::<Test>::get(&claim), Some((1, frame_system::Pallet::<Test>::block_number())));
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::<u8, ConstU32<10>>::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone().to_vec());

        assert_noop!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone().to_vec()), Error::<Test>::ProofAlreadyExist);
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::<u8, ConstU32<10>>::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone().to_vec());
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone().to_vec()));
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::<u8, ConstU32<10>>::try_from(vec![0, 1]).unwrap();

        assert_noop!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone().to_vec()), Error::<Test>::ClaimNotExist);
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::<u8, ConstU32<10>>::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone().to_vec());

        assert_noop!(PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone().to_vec()), Error::<Test>::NotClaimOwner);
    })
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::<u8, ConstU32<10>>::try_from(vec![0, 1]).unwrap();
        let dest = frame_system::ensure_signed(RuntimeOrigin::signed(2)).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone().to_vec());
        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone().to_vec(), dest));
    })
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::<u8, ConstU32<10>>::try_from(vec![0, 1]).unwrap();
        let dest = frame_system::ensure_signed(RuntimeOrigin::signed(2)).unwrap();
        assert_noop!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone().to_vec(), dest), Error::<Test>::ClaimNotExist);
    })
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::<u8, ConstU32<10>>::try_from(vec![0, 1]).unwrap();
        let dest = frame_system::ensure_signed(RuntimeOrigin::signed(2)).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone().to_vec());
        assert_noop!(PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone().to_vec(), dest), Error::<Test>::NotClaimOwner);
    })
}

