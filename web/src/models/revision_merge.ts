export interface Divergence {
    index: number
    existingRevisionHash: string | null
    upcomingRevisionHash: string | null
}

export interface RevisionsComparisonResult {
    divergences: Divergence[]
    mergedArray: string[]
    identical: boolean
    sameLength: boolean
    existingRevisionsLength: number
    upcomingRevisionsLength: number
}
