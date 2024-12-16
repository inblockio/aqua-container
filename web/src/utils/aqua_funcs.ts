import { Divergence, RevisionsComparisonResult } from "../models/revision_merge";

export function analyzeAndMergeRevisions(existingRevisions: string[], upcomingRevisions: string[]): RevisionsComparisonResult {
    // Sort the arrays
    const sortedExistingRevisions = [...existingRevisions] //.sort();
    const sortedUpcomingRevisions = [...upcomingRevisions] //.sort();

    // Check for divergence
    const divergences: Divergence[] = [];
    const maxLength = Math.max(sortedExistingRevisions.length, sortedUpcomingRevisions.length);

    for (let i = 0; i < maxLength; i++) {
        const existingRevisionHash = sortedExistingRevisions[i] || null;
        const upcomingRevisionHash = sortedUpcomingRevisions[i] || null;

        if (existingRevisionHash !== upcomingRevisionHash) {
            divergences.push({ index: i, existingRevisionHash, upcomingRevisionHash });
        }
    }

    // Merge arrays without duplicates
    const mergedArray = Array.from(new Set([...sortedExistingRevisions, ...sortedUpcomingRevisions]));

    // Properties to check
    const identical = divergences.length === 0; // True if no divergences
    const sameLength = sortedExistingRevisions.length === sortedUpcomingRevisions.length;

    // Return results
    return {
        divergences,
        mergedArray,
        identical,
        sameLength,
        existingRevisionsLength: sortedExistingRevisions.length,
        upcomingRevisionsLength: sortedUpcomingRevisions.length,
    };
}