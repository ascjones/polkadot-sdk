// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use codec::{Compact, Encode};
use scale_info::{
	form::{Form, MetaForm, PortableForm},
	prelude::{collections::BTreeMap, vec::Vec},
	IntoPortable, Registry,
};

/// The intermediate representation for the runtime metadata.
/// Contains the needed context that allows conversion to multiple metadata versions.
///
/// # Note
///
/// Further fields could be added or removed to ensure proper conversion.
/// When the IR does not contain enough information to generate a specific version
/// of the runtime metadata an appropriate default value is used (ie, empty vector).
pub struct MetadataIR<T: Form = MetaForm> {
	/// Pallet metadata.
	pub pallets: Vec<PalletMetadataIR<T>>,
	/// Metadata of the extrinsic.
	pub extrinsic: ExtrinsicMetadataIR<T>,
	/// The type of the `Runtime`.
	pub ty: T::Type,
	/// Metadata of the Runtime API.
	pub apis: Vec<RuntimeApiMetadataIR<T>>,
	/// The outer enums types as found in the runtime.
	pub outer_enums: OuterEnumsIR<T>,
	/// Metadata of view function queries
	pub view_functions: RuntimeViewFunctionsIR<T>,
}

/// Metadata of a runtime trait.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct RuntimeApiMetadataIR<T: Form = MetaForm> {
	/// Trait name.
	pub name: T::String,
	/// Trait methods.
	pub methods: Vec<RuntimeApiMethodMetadataIR<T>>,
	/// Trait documentation.
	pub docs: Vec<T::String>,
	/// Deprecation info
	pub deprecation_info: DeprecationStatusIR<T>,
}

impl IntoPortable for RuntimeApiMetadataIR {
	type Output = RuntimeApiMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		RuntimeApiMetadataIR {
			name: self.name.into_portable(registry),
			methods: registry.map_into_portable(self.methods),
			docs: registry.map_into_portable(self.docs),
			deprecation_info: self.deprecation_info.into_portable(registry),
		}
	}
}

/// Metadata of a runtime method.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct RuntimeApiMethodMetadataIR<T: Form = MetaForm> {
	/// Method name.
	pub name: T::String,
	/// Method parameters.
	pub inputs: Vec<RuntimeApiMethodParamMetadataIR<T>>,
	/// Method output.
	pub output: T::Type,
	/// Method documentation.
	pub docs: Vec<T::String>,
	/// Deprecation info
	pub deprecation_info: DeprecationStatusIR<T>,
}

impl IntoPortable for RuntimeApiMethodMetadataIR {
	type Output = RuntimeApiMethodMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		RuntimeApiMethodMetadataIR {
			name: self.name.into_portable(registry),
			inputs: registry.map_into_portable(self.inputs),
			output: registry.register_type(&self.output),
			docs: registry.map_into_portable(self.docs),
			deprecation_info: self.deprecation_info.into_portable(registry),
		}
	}
}

/// Metadata of a runtime method parameter.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct RuntimeApiMethodParamMetadataIR<T: Form = MetaForm> {
	/// Parameter name.
	pub name: T::String,
	/// Parameter type.
	pub ty: T::Type,
}

impl IntoPortable for RuntimeApiMethodParamMetadataIR {
	type Output = RuntimeApiMethodParamMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		RuntimeApiMethodParamMetadataIR {
			name: self.name.into_portable(registry),
			ty: registry.register_type(&self.ty),
		}
	}
}

/// Metadata of the the runtime query dispatch.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct RuntimeViewFunctionsIR<T: Form = MetaForm> {
	/// The type implementing the runtime query dispatch.
	pub ty: T::Type,
	/// The query interfaces metadata.
	pub interfaces: Vec<ViewFunctionsInterfaceIR<T>>,
}

/// Metadata of a runtime query interface.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct ViewFunctionsInterfaceIR<T: Form = MetaForm> {
	/// Name of the query interface.
	pub name: T::String,
	/// Queries belonging to the query interface.
	pub queries: Vec<ViewFunctionMetadataIR<T>>,
	/// Query interface documentation.
	pub docs: Vec<T::String>,
}

impl IntoPortable for ViewFunctionsInterfaceIR {
	type Output = ViewFunctionsInterfaceIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		ViewFunctionsInterfaceIR {
			name: self.name.into_portable(registry),
			queries: registry.map_into_portable(self.queries),
			docs: registry.map_into_portable(self.docs),
		}
	}
}

/// Metadata of a runtime view function.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct ViewFunctionMetadataIR<T: Form = MetaForm> {
	/// Query name.
	pub name: T::String,
	/// Query id.
	pub id: [u8; 32],
	/// Query args.
	pub args: Vec<QueryArgMetadataIR<T>>,
	/// Query output.
	pub output: T::Type,
	/// Query documentation.
	pub docs: Vec<T::String>,
}

impl IntoPortable for ViewFunctionMetadataIR {
	type Output = ViewFunctionMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		ViewFunctionMetadataIR {
			name: self.name.into_portable(registry),
			id: self.id,
			args: registry.map_into_portable(self.args),
			output: registry.register_type(&self.output),
			docs: registry.map_into_portable(self.docs),
		}
	}
}

/// Metadata of a runtime method argument.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct QueryArgMetadataIR<T: Form = MetaForm> {
	/// Query argument name.
	pub name: T::String,
	/// Query argument type.
	pub ty: T::Type,
}

impl IntoPortable for QueryArgMetadataIR {
	type Output = QueryArgMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		QueryArgMetadataIR {
			name: self.name.into_portable(registry),
			ty: registry.register_type(&self.ty),
		}
	}
}

/// The intermediate representation for a pallet metadata.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct PalletMetadataIR<T: Form = MetaForm> {
	/// Pallet name.
	pub name: T::String,
	/// Pallet storage metadata.
	pub storage: Option<PalletStorageMetadataIR<T>>,
	/// Pallet calls metadata.
	pub calls: Option<PalletCallMetadataIR<T>>,
	/// Pallet event metadata.
	pub event: Option<PalletEventMetadataIR<T>>,
	/// Pallet constants metadata.
	pub constants: Vec<PalletConstantMetadataIR<T>>,
	/// Pallet error metadata.
	pub error: Option<PalletErrorMetadataIR<T>>,
	/// Config's trait associated types.
	pub associated_types: Vec<PalletAssociatedTypeMetadataIR<T>>,
	/// Define the index of the pallet, this index will be used for the encoding of pallet event,
	/// call and origin variants.
	pub index: u8,
	/// Pallet documentation.
	pub docs: Vec<T::String>,
	/// Deprecation info
	pub deprecation_info: DeprecationStatusIR<T>,
}

impl IntoPortable for PalletMetadataIR {
	type Output = PalletMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletMetadataIR {
			name: self.name.into_portable(registry),
			storage: self.storage.map(|storage| storage.into_portable(registry)),
			calls: self.calls.map(|calls| calls.into_portable(registry)),
			event: self.event.map(|event| event.into_portable(registry)),
			constants: registry.map_into_portable(self.constants),
			error: self.error.map(|error| error.into_portable(registry)),
			associated_types: registry.map_into_portable(self.associated_types),
			index: self.index,
			docs: registry.map_into_portable(self.docs),
			deprecation_info: self.deprecation_info.into_portable(registry),
		}
	}
}

/// Metadata of the extrinsic used by the runtime.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct ExtrinsicMetadataIR<T: Form = MetaForm> {
	/// The type of the extrinsic.
	///
	/// Note: Field used for metadata V14 only.
	pub ty: T::Type,
	/// Extrinsic version.
	pub version: u8,
	/// The type of the address that signs the extrinsic
	pub address_ty: T::Type,
	/// The type of the outermost Call enum.
	pub call_ty: T::Type,
	/// The type of the extrinsic's signature.
	pub signature_ty: T::Type,
	/// The type of the outermost Extra/Extensions enum.
	// TODO: metadata-v16: remove this, the `implicit` type can be found in `extensions::implicit`.
	pub extra_ty: T::Type,
	/// The transaction extensions in the order they appear in the extrinsic.
	pub extensions: Vec<TransactionExtensionMetadataIR<T>>,
}

impl IntoPortable for ExtrinsicMetadataIR {
	type Output = ExtrinsicMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		ExtrinsicMetadataIR {
			ty: registry.register_type(&self.ty),
			version: self.version,
			address_ty: registry.register_type(&self.address_ty),
			call_ty: registry.register_type(&self.call_ty),
			signature_ty: registry.register_type(&self.signature_ty),
			extra_ty: registry.register_type(&self.extra_ty),
			extensions: registry.map_into_portable(self.extensions),
		}
	}
}

/// Metadata of a pallet's associated type.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct PalletAssociatedTypeMetadataIR<T: Form = MetaForm> {
	/// The name of the associated type.
	pub name: T::String,
	/// The type of the associated type.
	pub ty: T::Type,
	/// The documentation of the associated type.
	pub docs: Vec<T::String>,
}

impl IntoPortable for PalletAssociatedTypeMetadataIR {
	type Output = PalletAssociatedTypeMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletAssociatedTypeMetadataIR {
			name: self.name.into_portable(registry),
			ty: registry.register_type(&self.ty),
			docs: registry.map_into_portable(self.docs),
		}
	}
}

/// Metadata of an extrinsic's signed extension.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct TransactionExtensionMetadataIR<T: Form = MetaForm> {
	/// The unique signed extension identifier, which may be different from the type name.
	pub identifier: T::String,
	/// The type of the signed extension, with the data to be included in the extrinsic.
	pub ty: T::Type,
	/// The type of the implicit data, with the data to be included in the signed payload.
	pub implicit: T::Type,
}

impl IntoPortable for TransactionExtensionMetadataIR {
	type Output = TransactionExtensionMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		TransactionExtensionMetadataIR {
			identifier: self.identifier.into_portable(registry),
			ty: registry.register_type(&self.ty),
			implicit: registry.register_type(&self.implicit),
		}
	}
}

/// All metadata of the pallet's storage.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
/// The common prefix used by all storage entries.
pub struct PalletStorageMetadataIR<T: Form = MetaForm> {
	/// The common prefix used by all storage entries.
	pub prefix: T::String,
	/// Metadata for all storage entries.
	pub entries: Vec<StorageEntryMetadataIR<T>>,
}

impl IntoPortable for PalletStorageMetadataIR {
	type Output = PalletStorageMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletStorageMetadataIR {
			prefix: self.prefix.into_portable(registry),
			entries: registry.map_into_portable(self.entries),
		}
	}
}

/// Metadata about one storage entry.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct StorageEntryMetadataIR<T: Form = MetaForm> {
	/// Variable name of the storage entry.
	pub name: T::String,
	/// An `Option` modifier of that storage entry.
	pub modifier: StorageEntryModifierIR,
	/// Type of the value stored in the entry.
	pub ty: StorageEntryTypeIR<T>,
	/// Default value (SCALE encoded).
	pub default: Vec<u8>,
	/// Storage entry documentation.
	pub docs: Vec<T::String>,
	/// Deprecation info
	pub deprecation_info: DeprecationStatusIR<T>,
}

impl IntoPortable for StorageEntryMetadataIR {
	type Output = StorageEntryMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		StorageEntryMetadataIR {
			name: self.name.into_portable(registry),
			modifier: self.modifier,
			ty: self.ty.into_portable(registry),
			default: self.default,
			docs: registry.map_into_portable(self.docs),
			deprecation_info: self.deprecation_info.into_portable(registry),
		}
	}
}

/// A storage entry modifier indicates how a storage entry is returned when fetched and what the
/// value will be if the key is not present. Specifically this refers to the "return type" when
/// fetching a storage entry, and what the value will be if the key is not present.
///
/// `Optional` means you should expect an `Option<T>`, with `None` returned if the key is not
/// present. `Default` means you should expect a `T` with the default value of default if the key is
/// not present.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub enum StorageEntryModifierIR {
	/// The storage entry returns an `Option<T>`, with `None` if the key is not present.
	Optional,
	/// The storage entry returns `T::Default` if the key is not present.
	Default,
}

/// Hasher used by storage maps
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub enum StorageHasherIR {
	/// 128-bit Blake2 hash.
	Blake2_128,
	/// 256-bit Blake2 hash.
	Blake2_256,
	/// Multiple 128-bit Blake2 hashes concatenated.
	Blake2_128Concat,
	/// 128-bit XX hash.
	Twox128,
	/// 256-bit XX hash.
	Twox256,
	/// Multiple 64-bit XX hashes concatenated.
	Twox64Concat,
	/// Identity hashing (no hashing).
	Identity,
}

/// A type of storage value.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub enum StorageEntryTypeIR<T: Form = MetaForm> {
	/// Plain storage entry (just the value).
	Plain(T::Type),
	/// A storage map.
	Map {
		/// One or more hashers, should be one hasher per key element.
		hashers: Vec<StorageHasherIR>,
		/// The type of the key, can be a tuple with elements for each of the hashers.
		key: T::Type,
		/// The type of the value.
		value: T::Type,
	},
}

impl IntoPortable for StorageEntryTypeIR {
	type Output = StorageEntryTypeIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		match self {
			Self::Plain(plain) => StorageEntryTypeIR::Plain(registry.register_type(&plain)),
			Self::Map { hashers, key, value } => StorageEntryTypeIR::Map {
				hashers,
				key: registry.register_type(&key),
				value: registry.register_type(&value),
			},
		}
	}
}

/// Metadata for all calls in a pallet
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct PalletCallMetadataIR<T: Form = MetaForm> {
	/// The corresponding enum type for the pallet call.
	pub ty: T::Type,
	/// Deprecation status of the pallet call
	pub deprecation_info: DeprecationInfoIR<T>,
}

impl IntoPortable for PalletCallMetadataIR {
	type Output = PalletCallMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletCallMetadataIR {
			ty: registry.register_type(&self.ty),
			deprecation_info: self.deprecation_info.into_portable(registry),
		}
	}
}

/// Metadata about the pallet Event type.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct PalletEventMetadataIR<T: Form = MetaForm> {
	/// The Event type.
	pub ty: T::Type,
	/// Deprecation info of the event
	pub deprecation_info: DeprecationInfoIR<T>,
}

impl IntoPortable for PalletEventMetadataIR {
	type Output = PalletEventMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletEventMetadataIR {
			ty: registry.register_type(&self.ty),
			deprecation_info: self.deprecation_info.into_portable(registry),
		}
	}
}

/// Metadata about one pallet constant.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct PalletConstantMetadataIR<T: Form = MetaForm> {
	/// Name of the pallet constant.
	pub name: T::String,
	/// Type of the pallet constant.
	pub ty: T::Type,
	/// Value stored in the constant (SCALE encoded).
	pub value: Vec<u8>,
	/// Documentation of the constant.
	pub docs: Vec<T::String>,
	/// Deprecation info
	pub deprecation_info: DeprecationStatusIR<T>,
}

impl IntoPortable for PalletConstantMetadataIR {
	type Output = PalletConstantMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletConstantMetadataIR {
			name: self.name.into_portable(registry),
			ty: registry.register_type(&self.ty),
			value: self.value,
			docs: registry.map_into_portable(self.docs),
			deprecation_info: self.deprecation_info.into_portable(registry),
		}
	}
}

/// Metadata about a pallet error.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct PalletErrorMetadataIR<T: Form = MetaForm> {
	/// The error type information.
	pub ty: T::Type,
	/// Deprecation info
	pub deprecation_info: DeprecationInfoIR<T>,
}

impl IntoPortable for PalletErrorMetadataIR {
	type Output = PalletErrorMetadataIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletErrorMetadataIR {
			ty: registry.register_type(&self.ty),
			deprecation_info: self.deprecation_info.into_portable(registry),
		}
	}
}

/// The type of the outer enums.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct OuterEnumsIR<T: Form = MetaForm> {
	/// The type of the outer `RuntimeCall` enum.
	pub call_enum_ty: T::Type,
	/// The type of the outer `RuntimeEvent` enum.
	pub event_enum_ty: T::Type,
	/// The module error type of the
	/// [`DispatchError::Module`](https://docs.rs/sp-runtime/24.0.0/sp_runtime/enum.DispatchError.html#variant.Module) variant.
	///
	/// The `Module` variant will be 5 scale encoded bytes which are normally decoded into
	/// an `{ index: u8, error: [u8; 4] }` struct. This type ID points to an enum type which
	/// instead interprets the first `index` byte as a pallet variant, and the remaining `error`
	/// bytes as the appropriate `pallet::Error` type. It is an equally valid way to decode the
	/// error bytes, and can be more informative.
	///
	/// # Note
	///
	/// - This type cannot be used directly to decode `sp_runtime::DispatchError` from the chain.
	///   It provides just the information needed to decode `sp_runtime::DispatchError::Module`.
	/// - Decoding the 5 error bytes into this type will not always lead to all of the bytes being
	///   consumed; many error types do not require all of the bytes to represent them fully.
	pub error_enum_ty: T::Type,
}

impl IntoPortable for OuterEnumsIR {
	type Output = OuterEnumsIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		OuterEnumsIR {
			call_enum_ty: registry.register_type(&self.call_enum_ty),
			event_enum_ty: registry.register_type(&self.event_enum_ty),
			error_enum_ty: registry.register_type(&self.error_enum_ty),
		}
	}
}

/// Deprecation status for an entry inside MetadataIR
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub enum DeprecationStatusIR<T: Form = MetaForm> {
	/// Entry is not deprecated
	NotDeprecated,
	/// Deprecated without a note.
	DeprecatedWithoutNote,
	/// Entry is deprecated with an note and an optional `since` field.
	Deprecated {
		/// Note explaining the deprecation
		note: T::String,
		/// Optional value for denoting version when the deprecation occured
		since: Option<T::String>,
	},
}
impl IntoPortable for DeprecationStatusIR {
	type Output = DeprecationStatusIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		match self {
			Self::Deprecated { note, since } => {
				let note = note.into_portable(registry);
				let since = since.map(|x| x.into_portable(registry));
				DeprecationStatusIR::Deprecated { note, since }
			},
			Self::DeprecatedWithoutNote => DeprecationStatusIR::DeprecatedWithoutNote,
			Self::NotDeprecated => DeprecationStatusIR::NotDeprecated,
		}
	}
}
/// Deprecation info for an enums/errors/calls.
/// Denotes full/partial deprecation of the type
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub enum DeprecationInfoIR<T: Form = MetaForm> {
	/// Type is not deprecated
	NotDeprecated,
	/// Entry is fully deprecated.
	ItemDeprecated(DeprecationStatusIR<T>),
	/// Entry is partially deprecated.
	VariantsDeprecated(BTreeMap<Compact<u8>, DeprecationStatusIR<T>>),
}
impl IntoPortable for DeprecationInfoIR {
	type Output = DeprecationInfoIR<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		match self {
			Self::VariantsDeprecated(entries) => {
				let entries =
					entries.into_iter().map(|(k, entry)| (k, entry.into_portable(registry)));
				DeprecationInfoIR::VariantsDeprecated(entries.collect())
			},
			Self::ItemDeprecated(deprecation) =>
				DeprecationInfoIR::ItemDeprecated(deprecation.into_portable(registry)),
			Self::NotDeprecated => DeprecationInfoIR::NotDeprecated,
		}
	}
}
