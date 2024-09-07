//如果 std 特性没有启用（即 not(feature = "std") 为真），则编译器会应用 no_std 属性，使得代码在没有标准库的环境中编译。
#![cfg_attr(not(feature = "std"), no_std)]

// 将 pallet 模块的所有公共项引入当前模块的作用域，并将这些项导出到当前模块的外部。
// 使得其他模块可以通过当前模块访问 pallet 的项。
pub use pallet::*;

// 当运行 cargo test 时，Rust 编译器会启用 test 配置，因此被 #[cfg(test)] 注解的代码块会被编译。
// 当运行 cargo build 或其他非测试构建命令时，这些代码块不会被编译，从而避免了将测试代码包含在生产构建中。
// mod mock; 用于声明一个名为 mock 的模块。
// 将 mod mock; 放在 #[cfg(test)] 注解的代码块中，意味着 mock 模块仅在测试构建时被编译。
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

/*
#[frame_support::pallet]
用途:

#[frame_support::pallet] 是 Substrate 提供的宏，用于标记一个模块作为 pallet 的定义。
这个宏将自动为模块提供一些必要的 trait 实现和功能，使其成为 Substrate 的一个正式 pallet。
功能:

使用这个宏可以省去手动实现一些标准的 trait 和功能，从而简化 pallet 的定义过程。
#[frame_support::pallet] 宏会处理 pallet 的注册、配置以及其他必要的框架设置，使得该模块能够与 Substrate 的运行时系统集成。

pub mod pallet {}
用途:

pub mod pallet {} 定义了一个名为 pallet 的模块。这个模块的内容将构成实际的 pallet 实现。
pub 关键字表示该模块是公共的，允许在 crate 的其他地方或在其他 crate 中访问这个模块。
作用:

在 Substrate 项目中，pallet 模块通常包含了 pallet 的核心逻辑，例如存储项、事件、调用等。
在 pallet 模块中定义的功能会被 Substrate 的 runtime 使用，并提供具体的区块链逻辑。
*/
#[frame_support::pallet]
pub mod pallet {
    use super::*;    // 引入父模块中的所有公共项
    use frame_support::{ensure, pallet_prelude::*};
    use frame_system::{ensure_signed, pallet_prelude::*};

    /*
    // 定义结构体
    #[pallet::pallet] 宏
    用途:

    #[pallet::pallet] 是 Substrate 提供的宏，用于标记一个结构体作为 pallet 的定义。
    这个宏会处理一些 boilerplate 代码，使得结构体能够与 Substrate 的 runtime 系统集成。
    功能:

    宏为 pallet 提供了必要的 trait 实现，例如注册 pallet、处理存储项、事件等。
    简化了 pallet 的定义过程，自动为你生成一些标准代码，避免手动实现繁琐的细节。
    */
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /*
    // 定义接口
    定义Config 接口，并且继承frame_system::Config中的相关方法
    */
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /*
        定义RuntimeEvent类型别名, 要有后面的From和IsType约束
        From<Event<Self>>: 要求 RuntimeEvent 能够从 pallet 中定义的事件类型 Event<Self> 转换过来。
        IsType<<Self as frame_system::Config>::RuntimeEvent>: 确保 RuntimeEvent 类型与 Substrate 运行时事件类型兼容，保证事件的兼
容性和正确处理。
        */
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /*
        #[pallet::constant] 宏用于标记一个类型为常量类型。常量类型的值在编译时是固定的，不会在运行时变化。
        type MaxClaimLength: Get<u32>; 意味着 MaxClaimLength 是一个常量类型，它的值是 u32 类型的，并且可以通过 Get trait 的 get >方法访问。
        */
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
    }
    /*
    1. [pallet::storage]
    #[pallet::storage] 宏用于定义 pallet 的存储项。存储项是 pallet 用来存储状态的地方，比如数据、映射、集合等。

    2. #[pallet::getter(fn proofs)]
    用途:
    #[pallet::getter(fn proofs)] 宏用于自动生成一个 getter 方法，用于访问存储项的值。
    功能:
    fn proofs 是自动生成的 getter 方法的名称，可以通过这个方法来读取存储项中的数据。例如，你可以通过 Pallet::<T>::proofs(key) 来>访问存储项 Proofs 中的值。
    */
    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,                                  // storage prefix
        Blake2_128Concat,                   // hash function
        BoundedVec<u8, T::MaxClaimLength>,  // 存储项的键类型。键是一个 BoundedVec，它是一个具有最大长度限制的向量，元素类型是 u8。T::MaxClaimLength 指定了 BoundedVec 的最大长度。
        (T::AccountId, BlockNumberFor<T>)   // 存储项的值类型。值是一个元组，包含 T::AccountId（账户 ID）和 BlockNumberFor<T>（区块编号）
    >;

    /*
    1. #[pallet::event]
    作用:
    #[pallet::event] 宏用于标记一个枚举为 pallet 的事件类型。它告诉 Substrate 这个枚举包含了 pallet 中所有的事件。
    功能:
    该宏生成了与事件相关的代码，使得事件能够被存储和触发。它还会自动处理事件的序列化和反序列化。
    2. #[pallet::generate_deposit(pub(super) fn deposit_event)]
    作用:
    #[pallet::generate_deposit(pub(super) fn deposit_event)] 宏用于生成一个事件存储方法 deposit_event，并设置其访问级别为 pub(super)。
    功能:
    生成的 deposit_event 方法用于将事件存储到 Substrate 的链上事件日志中。这个方法会被 pallet 的调用者用来触发事件。
    pub(super) 访问修饰符使得 deposit_event 方法在当前模块内可见，但对外部不可见。这种访问修饰符可以确保 deposit_event 只被当前模
块内部的代码使用。
    */
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimTransfered(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimLengthTooLarge,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}  

    /*
    1. #[pallet::call] 宏用于标记一个 impl 块，指示其中的方法是 pallet 的调度方法。调度方法是用户可以调用的公共接口，用于与 pallet 交互，执行特定的操作或交易。
    功能: 该宏帮助生成和管理与调度方法相关的 boilerplate 代码，比如方法的签名、权重、调用条件等。
    */    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight({0})]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            /*
            ensure_signed 是一个来自 frame_support 模块的实用函数，用于在 Substrate 的 pallet 中确保调用方法的交易是由一个签名账>户发起的。
            它的作用是验证 origin 是否是一个签名的交易，并提取签名者的身份（即账户 ID）。如果 origin 不是签名交易，ensure_signed 会返回错误。
            */
            ensure!(claim.len() <= T::MaxClaimLength::get() as usize, Error::<T>::ClaimLengthTooLarge);
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            Proofs::<T>::insert(
                &claim,
                (sender.clone(), frame_system::Pallet::<T>::block_number())
            );

            Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight({0})]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender, claim));

            Ok(().into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight({0})]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
            target: T::AccountId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::insert(
                &claim,
                (target.clone(), frame_system::Pallet::<T>::block_number())
            );

            Self::deposit_event(Event::ClaimTransfered(sender, target, claim));

            Ok(().into())
        }
    }
}
