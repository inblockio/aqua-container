use crate::models::database_models::RevisionTable;
use crate::models::database_models::LatestTable;
use crate::schema::Latest;
use crate::schema::Revision;
use crate::schema::Revision::dsl::*;
use crate::schema::Latest::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;

pub fn insert_revision(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_revision: RevisionTable,
) -> Result<(), Error> {
    // Insert the revision
    let revision_result = diesel::insert_into(Revision)
        .values(&new_revision)
        .execute(conn);

    if revision_result.is_err() {
        let rev= revision_result.err().unwrap();
        println!(
            "Error inserting revision: {:?}",
            rev.to_string().clone()
        );
        return Err(rev);
    }

    // Save to `Latest` table
    let latest_entry = LatestTable {
        hash: new_revision.hash.clone(),
        owner: new_revision.owner.clone(),
    };

    let latest_entry_result = diesel::insert_into(Latest)
        .values(&latest_entry)
        .execute(conn);

    if latest_entry_result.is_err() {
        let latest_entry_err = latest_entry_result.err().unwrap();
        println!(
            "Error inserting latest entry: {:?}",
            latest_entry_err.to_string().clone()
        );
        return Err(latest_entry_err);
    }

    Ok(())
}

pub fn delete_revision(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    revision_hash: String,
) -> Result<(), Error> {
    
    // Find the revision by hash
    let revision: RevisionTable = Revision.filter( crate::schema::Revision::dsl::hash.eq(&revision_hash)).first(conn)?;

    // Remove the revision
    diesel::delete(Revision.filter(crate::schema::Revision::dsl::hash.eq(&revision_hash))).execute(conn)?;

    // Handle the `Latest` table
    if let Some(prev_hash) = revision.previous {
        // Check if the `previous` hash exists in `Revision`
        let exists: bool = Revision
            .filter(crate::schema::Revision::dsl::hash.eq(&prev_hash))
            .select(diesel::dsl::count_star())
            .get_result::<i64>(conn)?
            > 0;

        if exists {
            // Update the `Latest` table with the previous revision
            diesel::update(Latest.filter(crate::schema::Latest::dsl::hash.eq(&revision_hash)))
                .set(crate::schema::Latest::dsl::hash.eq(prev_hash))
                .execute(conn)?;
        } else {
            // Delete from `Latest` if no previous revision exists
            diesel::delete(Latest.filter(crate::schema::Latest::dsl::hash.eq(&revision_hash))).execute(conn)?;
        }
    } else {
        // Delete from `Latest` if no `previous` exists
        diesel::delete(Latest.filter(crate::schema::Latest::dsl::hash.eq(&revision_hash))).execute(conn)?;
    }

    Ok(())
}
