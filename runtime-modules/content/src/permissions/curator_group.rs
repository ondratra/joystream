use super::*;

/// A group, that consists of `curators` set
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Eq, PartialEq, Clone, Debug)]
pub struct CuratorGroup<T: Trait> {
    /// Curators set, associated with a iven curator group
    curators: BTreeSet<T::CuratorId>,

    /// When `false`, curator in a given group is forbidden to act
    active: bool,

    /// Used to count the number of Channels, given curator group owns
    number_of_channels_owned: u32,
}

impl<T: Trait> Default for CuratorGroup<T> {
    fn default() -> Self {
        Self {
            curators: BTreeSet::new(),
            // default curator group status right after creation
            active: false,
            number_of_channels_owned: 0,
        }
    }
}

impl<T: Trait> CuratorGroup<T> {
    /// Check if `CuratorGroup` contains curator under given `curator_id`
    pub fn is_curator(&self, curator_id: &T::CuratorId) -> bool {
        self.curators.contains(curator_id)
    }

    /// Check if `CuratorGroup` is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_number_of_channels_owned(&self) -> u32 {
        self.number_of_channels_owned
    }

    /// Set `CuratorGroup` status as provided
    pub fn set_status(&mut self, is_active: bool) {
        self.active = is_active
    }

    /// Retrieve set of all curator_ids related to `CuratorGroup` by reference
    pub fn get_curators(&self) -> &BTreeSet<T::CuratorId> {
        &self.curators
    }

    /// Retrieve set of all curator_ids related to `CuratorGroup` by mutable  reference
    pub fn get_curators_mut(&mut self) -> &mut BTreeSet<T::CuratorId> {
        &mut self.curators
    }

    /// Increment number of channels `CuratorGroup` owns
    pub fn increment_number_of_channels_owned_count(&mut self) {
        self.number_of_channels_owned += 1;
    }

    /// Decrement number of channels `CuratorGroup` owns
    pub fn decrement_number_of_channels_owned_count(&mut self) {
        self.number_of_channels_owned -= 1;
    }

    /// Ensure curator group does not maintain any `Channel`
    pub fn ensure_curator_group_owns_no_channels(&self) -> Result<(), Error<T>> {
        ensure!(
            self.number_of_channels_owned == 0,
            Error::<T>::CuratorGroupRemovalForbidden
        );
        Ok(())
    }

    /// Ensure `MaxNumberOfCuratorsPerGroup` constraint satisfied
    pub fn ensure_max_number_of_curators_limit_not_reached(&self) -> Result<(), Error<T>> {
        ensure!(
            self.curators.len() < T::MaxNumberOfCuratorsPerGroup::get() as usize,
            Error::<T>::CuratorsPerGroupLimitReached
        );
        Ok(())
    }

    /// Ensure curator under given `curator_id` exists in `CuratorGroup`
    pub fn ensure_curator_in_group_exists(
        &self,
        curator_id: &T::CuratorId,
    ) -> Result<(), Error<T>> {
        ensure!(
            self.get_curators().contains(curator_id),
            Error::<T>::CuratorIsNotAMemberOfGivenCuratorGroup
        );
        Ok(())
    }

    /// Ensure curator under given `curator_id` does not exist yet in `CuratorGroup`
    pub fn ensure_curator_in_group_does_not_exist(
        &self,
        curator_id: &T::CuratorId,
    ) -> Result<(), Error<T>> {
        ensure!(
            !self.get_curators().contains(curator_id),
            Error::<T>::CuratorIsAlreadyAMemberOfGivenCuratorGroup
        );
        Ok(())
    }

    /// Authorize curator, performing all checks to ensure curator can act
    pub fn perform_curator_in_group_auth(
        curator_id: &T::CuratorId,
        curator_group_id: &T::CuratorGroupId,
        account_id: &T::AccountId,
    ) -> Result<(), Error<T>> {
        // Ensure curator authorization performed succesfully
        ensure_curator_auth_success::<T>(curator_id, account_id)?;

        // Retrieve corresponding curator group
        let curator_group = Module::<T>::curator_group_by_id(curator_group_id);

        // Ensure curator group is active
        ensure!(
            curator_group.is_active(),
            Error::<T>::CuratorGroupIsNotActive
        );

        // Ensure curator under given curator_id exists in CuratorGroup
        Self::ensure_curator_in_group_exists(&curator_group, curator_id)?;
        Ok(())
    }
}
