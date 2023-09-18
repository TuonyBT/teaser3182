
const RATIOS: [[u32; 2]; 4] = [[7, 3], [10, 7], [1, 11], [11, 19]];
const TOT: u32 = 182;

fn main() {

    let ss = RATIOS.iter().map(|[l, h]| l + h).collect::<Vec<u32>>();
    let ss_sum = ss.iter().sum::<u32>();
    let coef_lims = ss.iter().map(|s| 1..((TOT - ss_sum + s) / s + 1)).collect::<Vec<std::ops::Range<u32>>>();

    let mut stand_idx = 1;
    let mut pairings = vec![vec![[1_u32, 2]]];

    while stand_idx < coef_lims.len() {
        pairings = next_pairings(&pairings);    
        stand_idx += 1;
    }

    let mut stands: Vec<u32> = vec![0; coef_lims.len()];
    let stand_scores = search_coefs(&coef_lims, &mut stands, 0, &ss)
                    .into_iter().filter(|scores| scores.into_iter().sum::<u32>() == TOT)
                    .collect::<Vec<Vec<u32>>>();

    for stand in stand_scores {
        let coefs = stand.into_iter().zip(&ss)
                                    .map(|(sc, &ss)| sc / ss)
                                    .collect::<Vec<u32>>();

        let indiv_scores = coefs.into_iter().zip(RATIOS)
                            .map(|(c, [l, h])| [l * c, h * c])
                            .collect::<Vec<[u32; 2]>>();

        let batter_totals = pairings.iter()
                    .map(|pair| pair.clone().into_iter().zip(&indiv_scores)
                                                    .map(|([l_id, h_id], &[l_sc, h_sc])| [(l_id, l_sc), (h_id, h_sc)])
                                                    .flatten()
                                                    .collect::<Vec<(u32, u32)>>())
                    .collect::<Vec<Vec<(u32, u32)>>>();

        for test in batter_totals {
            let mut indiv_totals = [0_u32; 5];
            for (batter, score) in test {
                indiv_totals[batter as usize - 1] += score;
            }
            let max_index = indiv_totals.iter().enumerate().max_by_key(|&(_, &value)| value).map(|(index, _)| index);
            match max_index {
                Some(2) => {
                    println!("Individual totals for innings with third man as highest-scorer: {:?}", indiv_totals)
                },
                _ => print!(""),
            };
        }

    }
                

}





//  Search space for the possible coefficients that apply to the ratios in each stand
fn search_coefs(ranges: &[std::ops::Range<u32>], coords: &mut [u32], dim: usize, sums: &Vec<u32>) -> Vec<Vec<u32>> {
    if dim == ranges.len() {
        return vec![coords.to_vec()];
    }

    let mut result = Vec::new();

    for val in ranges[dim].to_owned() {
        coords[dim] = val * sums[dim];


        let check = coords[..dim + 1].iter().sum::<u32>();

        if check > TOT {
            coords[dim] = sums[dim];
            break;
        }

        let sub_results = search_coefs(ranges, coords, dim + 1, sums);
        result.extend(sub_results);
    }
    result
}

fn next_pairings(pairings: &Vec<Vec<[u32; 2]>>) -> Vec<Vec<[u32; 2]>> {

    let mut result = Vec::<Vec<[u32; 2]>>::new();

    for pairing in pairings {

        let new = match pairing.last() {
            Some(p) => p.iter().map(|&batter| [batter, p[1] + 1]).collect::<Vec<[u32; 2]>>(),
            None => Vec::<[u32; 2]>::new(),
        };

        for new_pair in new {
            let mut new_order = pairing.clone();
            new_order.push(new_pair);
            result.push(new_order);
        }

    }

    result


}