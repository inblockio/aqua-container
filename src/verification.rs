use std::collections::{BTreeMap, HashSet};
use aqua_verifier_rs_types::{crypt, models::{hash::Hash, public_key::PublicKey, revision::Revision, signature::Signature, timestamp::Timestamp, tx_hash::TxHash}};
use sha3::Digest;

use crate::revision_integrity::{RevisionIntegrity, HashChainIntegrity};




pub fn content_hash(content: &BTreeMap<String, String>) -> Hash {
    // 3.a create hasher {c}
    let mut c = crypt::Hasher::default();
    // 3.b iterate over rev.content.content by its keys
    for value in content.values() {
        // 3.c add each value of rev.content.content to hasher {c}
        c.update(value);
    }
    Hash::from(c.finalize())
}

pub fn metadata_hash(
    domain_id: &str,
    time_stamp: &Timestamp,
    previous_verification_hash: Option<&Hash>,
) -> Hash {
    // 4.a create hasher {m}
    let mut m = crypt::Hasher::default();
    // 4.b add rev.metadata.domain_id to hasher {m}
    m.update(domain_id);
    // 4.c add rev.metadata.time_stamp (in format %Y%m%d%H%M%S) to hasher {m}
    m.update(time_stamp.to_string());
    // 4.d if rev.metadata.previous_verification_hash exists then add rev.metadata.previous_verification_hash to hasher {m}
    if let Some(prev_verification_hash) = previous_verification_hash {
        m.update(prev_verification_hash.to_stackstr());
    }
    Hash::from(m.finalize())
}

pub fn verification_hash(
    content_hash: &Hash,
    metadata_hash: &Hash,
    signature_hash: Option<&Hash>,
    witness_hash: Option<&Hash>,
) -> Hash {
    let mut v = crypt::Hasher::default();
    // 5.b add rev.content.content_hash to hasher {v}
    v.update(content_hash.to_stackstr());
    // 5.c add rev.metadata.metadata_hash to hasher {v}
    v.update(metadata_hash.to_stackstr());
    // 5.d if prev?.signature exists then add prev.signature.signature_hash to hasher {v}
    if let Some(prev_signature_hash) = signature_hash {
        v.update(prev_signature_hash.to_stackstr());
    }
    // 5.e if prev?.witness exists then add prev.witness.witness_hash to hasher {v}
    if let Some(prev_witness_hash) = witness_hash {
        v.update(prev_witness_hash.to_stackstr());
    }
    Hash::from(v.finalize())
}

/// [a] verification_hash integrity
///
/// IMPORTANT: what does this verify?
/// - content and metadata have not been messed with
/// - this does not tell you anything about signature or witness
///
/// IMPORTANT: how to verify prev?
/// - recursively is possible but make sure not to get caught by a loop where two or more revisions point to one another as their prevs
///
/// prerequisites: rev, prev? [trusted verification_hash, signature_hash and witness_hash]
pub(super) fn only_verification_hash_integrity(
    rev: &Revision,
    prev: Option<&Revision>,
) -> flagset::FlagSet<RevisionIntegrity> {
    // use RevisionIntegrity::*;
    let mut integrity = flagset::FlagSet::default();

    // 1 previous_verification_hash
    // 1.a either both or neither of prev and rev.metadata.previous_verification_hash exist
    'prev_verification_hash: {
        if let Some(prev_verification_hash) = &rev.metadata.previous_verification_hash {
            let Some(prev) = prev else {
                integrity |= RevisionIntegrity::NoPrevRevision;
                break 'prev_verification_hash;
            };
            // 1.b if both exist then prev.metadata.verification_hash == rev.metadata.previous_verification_hash
            if prev.metadata.verification_hash != *prev_verification_hash {
                integrity |= RevisionIntegrity::PrevVerificationHashNotMatching;
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if prev.is_some() {
                integrity |= RevisionIntegrity::PrevVerificationHashNotMatching;
            }
        }
    }

    // 2 file_hash
    // 2.a make sure both or neither of rev.content.file and rev.content.content["file_hash"] exist
    // 2.b if neither exist move on to [3]
    'file_hash: {
        if let Some(file) = &rev.content.file {
            let Some(file_hash) = rev.content.content.get("file_hash") else {
                integrity |= RevisionIntegrity::FileHashNotMatching;
                break 'file_hash;
            };
            // 2.c create hasher {f}
            let mut f = crypt::Hasher::default();
            // 2.d add rev.content.file.data as raw data (not base64) to hasher {f}
            f.update(file.data.as_ref());
            // 2.e output of hasher {f} equals rev.content.content["file_hash"]
            //todo: should we instead try to parse the file_hash to a hash?
            // what about differently formatted hashes in file_hash?
            if Hash::from(f.finalize()).to_string() != file_hash.as_str() {
                integrity |= RevisionIntegrity::FileHashNotMatching;
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if rev.content.content.contains_key("file_hash") {
                integrity |= RevisionIntegrity::NoFile;
            }
        }
    }

    // 3 content_hash
    // 3.d output of hasher {c} equals rev.content.content_hash
    if content_hash(&rev.content.content) != rev.content.content_hash {
        integrity |= RevisionIntegrity::ContentHashNotMatching;
    }

    // 4 metadata_hash
    // 4.e output of hasher {m} equals rev.metadata.metadata_hash
    if metadata_hash(
        &rev.metadata.domain_id,
        &rev.metadata.time_stamp,
        rev.metadata.previous_verification_hash.as_ref(),
    ) != rev.metadata.metadata_hash
    {
        integrity |= RevisionIntegrity::MetadataHashNotMatching;
    }

    // 5 verification_hash
    // 5.a create hasher {v}
    // 5.f output of hasher {v} equals rev.metadata.verification_hash
    if verification_hash(
        &rev.content.content_hash,
        &rev.metadata.metadata_hash,
        rev.signature.as_ref().map(|s| &s.signature_hash),
        rev.witness.as_ref().map(|w| &w.witness_hash),
    ) != rev.metadata.verification_hash
    {
        integrity |= RevisionIntegrity::VerificationHashNotMatching;
    }

    integrity
}


pub fn signature_hash(signature: &Signature, public_key: &PublicKey) -> Hash {
    // 4.a create hasher {s}
    let mut s = crypt::Hasher::default();
    // 4.b add rev.signature.signature to hasher {s}
    s.update(signature.to_stackstr());
    // 4.c add rev.signature.public_key to hasher {s}
    s.update(public_key.to_stackstr());
    Hash::from(s.finalize())
}

/// [b] signature_hash integrity
///
/// IMPORTANT: what does this verify?
/// - it verifies that the revision's verification_hash has been signed using the listed public_key
/// - it verifies that the signature_hash hashed that signature with that public_key
///
/// prerequisites: rev [trusted verification_hash]
pub(super) fn only_signature_hash_integrity(rev: &Revision, prev: Option<&Revision>) -> flagset::FlagSet<RevisionIntegrity> {
    let mut integrity = flagset::FlagSet::default();

    let Some(sign) = &rev.signature else {
        return flagset::FlagSet::from(RevisionIntegrity::NoSignature);
    };
    let Some(prev_hash) = &prev.map(|a|a.metadata.verification_hash) else {
        return flagset::FlagSet::from(RevisionIntegrity::NoPrevRevision);
    };

    // 1 signature + recovery_id
    // 1.a deserialize the hex-encoded rev.signature.signature into exactly 65 bytes
    // 1.b declare first 64 bytes of decoded rev.signature.signature as signature
    // 1.c declare last byte of decoded rev.signature.signature as recovery_id
    // 1.d apply secp256k1 parse_standard to 64 byte signature
    // 1.e parse recovery_id as ethereum rpc format (make sure it is equal to 27, 28, 29 or 30)

    // 2 public_key
    // 2.a parse the hex-encoded rev.signature.public_key as secp256k1 public key

    // already done during parsing

    // 3 verifying the signature
    // 3.a create sha3 Keccak256 hasher {k}
    let mut k = crypt::Keccak256::default();
    // 3.b add "\x19Ethereum Signed Message:\n177I sign the following page verification_hash: [0x" to hasher {k}
    k.update("\x19Ethereum Signed Message:\n177I sign the following page verification_hash: [0x");
    // 3.c add rev.metadata.verification_hash to hasher {k}
    k.update(prev_hash.to_stackstr());
    // 3.d add "]" to hasher {k}
    k.update("]");
    // 3.e parse output of hasher {k} as secp256k1 message
    let message = libsecp256k1::Message::parse(&k.finalize().into());
    // 3.f recover public_key with secp256k1's recover using message, signature and recovery_id
    let Signature {
        signature,
        recovery_id,
    } = sign.signature;
    let Ok(public_key) = libsecp256k1::recover(&message, &signature, &recovery_id) else {
        return flagset::FlagSet::from(RevisionIntegrity::SignatureError);
    };
    // 3.g check equality of public_key and parsed public_key
    if public_key != *sign.public_key {
        integrity |= RevisionIntegrity::PublicKeyNotMatching;
    }

    // 4 signature_hash
    // 4.d output of hasher {s} equals rev.signature.signature_hash
    if signature_hash(&sign.signature, &sign.public_key) != sign.signature_hash {
        integrity |= RevisionIntegrity::SignatureHashNotMatching;
    }

    integrity
}

pub fn witness_hash(
    domain_snapshot_genesis_hash: &Hash,
    merkle_root: &Hash,
    witness_network: &str,
    witness_event_transaction_hash: &TxHash,
) -> Hash {
    // 2.a create hasher {w}
    let mut w = crypt::Hasher::default();
    // 2.b add rev.witness.domain_snapshot_genesis_hash to hasher {w}
    w.update(domain_snapshot_genesis_hash.to_stackstr());
    // 2.c add rev.witness.merkle_root to hasher {w}
    w.update(merkle_root.to_stackstr());
    // 2.d add rev.witness.witness_network to hasher {w}
    w.update(witness_network);
    // 2.e add rev.witness.witness_event_transaction_hash to hasher {w}
    w.update(witness_event_transaction_hash.to_stackstr());
    Hash::from(w.finalize())
}

/// [c] witness_hash integrity
///
/// IMPORTANT: what does this verify?
/// - the merkle_tree forms a merkle_tree
/// - the merkle_tree has merkle_root as its root node
/// - the merkle_tree contains this rev's verification_hash
/// - the witness_hash describes this merkle_tree publication
///
/// prerequisites: rev [trusted verification_hash]
pub(super) fn only_witness_hash_integrity(rev: &Revision, prev: Option<&Revision>) -> flagset::FlagSet<RevisionIntegrity> {
    let mut integrity = flagset::FlagSet::default();

    let Some(witness) = &rev.witness else {
        return flagset::FlagSet::from(RevisionIntegrity::NoWitness);
    };
    let Some(prev_hash) = &prev.map(|a|a.metadata.verification_hash) else {
        return flagset::FlagSet::from(RevisionIntegrity::NoPrevRevision);
    };

    // 1 merkle_tree
    // 1.a create set {a} "free leafs"
    let mut a = HashSet::<crypt::Hash>::new();
    // 1.b create set {b} "free roots"
    let mut b = HashSet::<crypt::Hash>::new();
    // 1.c create set {c} "matched"
    let mut c = HashSet::<crypt::Hash>::new();
    // 1.d add merkle_root to set {b}
    a.insert(*witness.merkle_root);
    // 1.e for each node in the merkle_tree
    for node in &witness.structured_merkle_proof {
        // 1.e.i    node.left_leaf is not in set {c} or set {a}
        if c.contains(&*node.left_leaf) || a.contains(&*node.left_leaf) {
            dbg!(&node.left_leaf);
            integrity |= RevisionIntegrity::DuplicateMerkleLeaf;
        }
        // 1.e.ii   node.right_leaf is not in set {c} or set {a}
        if c.contains(&*node.right_leaf) || a.contains(&*node.right_leaf) {
            dbg!(&node.right_leaf);
            integrity |= RevisionIntegrity::DuplicateMerkleLeaf;
        }
        // 1.e.iii  remove node.left_leaf from set {b} and insert into set {c} *or* insert into set {a}
        if b.remove(&*node.left_leaf) {
            c.insert(*node.left_leaf);
        } else {
            a.insert(*node.left_leaf);
        }
        // 1.e.iv   remove node.right_leaf from set {b} and insert into set {c} *or* insert into set {a}
        if b.remove(&*node.right_leaf) {
            c.insert(*node.right_leaf);
        } else {
            a.insert(*node.right_leaf);
        }
        // 1.e.v    create hasher {p}
        let mut p = crypt::Hasher::default();
        // 1.e.vi   add node.left_leaf to hasher {p}
        p.update(node.left_leaf.to_stackstr());
        // 1.e.vii  add node.right_leaf to hasher {p}
        p.update(node.right_leaf.to_stackstr());
        // 1.e.viii output of hasher {p} is not in set {c} or set {b}
        let p_output = p.finalize();
        if c.contains(&p_output) || b.contains(&p_output) {
            // dbg!(hash::Hash::from(p_output));
            integrity |= RevisionIntegrity::DuplicateMerkleLeaf;
        }
        // 1.e.ix   remove output of hasher {p} from set {a} and insert into set {c} *or* insert into set {b}
        if a.remove(&p_output) {
            c.insert(p_output);
        } else {
            b.insert(p_output);
        }
    }
    // 1.f verification_hash is in set {a}
    if !a.contains(&**prev_hash) {
        integrity |= RevisionIntegrity::VerificationHashNotInMerkleTree;
    }
    // 1.g set {b} is empty
    if !b.is_empty() {
        integrity |= RevisionIntegrity::MerkleTreeIncomplete;
    }
    // 1.h merkle_root is in set {c}
    if !c.contains(&*witness.merkle_root) {
        integrity |= RevisionIntegrity::MerkleTreeIncomplete;
    }
    // 2 witness_hash
    // 2.f output of hasher {w} equals rev.witness.witness_hash
    if witness_hash(
        &witness.domain_snapshot_genesis_hash,
        &witness.merkle_root,
        &witness.witness_network,
        &witness.witness_event_transaction_hash,
    ) != witness.witness_hash
    {
        integrity |= RevisionIntegrity::WitnessHashNotMatching;
    }

    integrity
}