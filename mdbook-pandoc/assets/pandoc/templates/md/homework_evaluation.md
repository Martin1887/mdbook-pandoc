---
title: "$lang_evaluation_title$"
...




# $lang_group_information$

\begin{minipage}{118mm}
\begin{tabular}{cp{52mm}p{36mm}}\hline
    Nr. & $lang_surname$ & $lang_first_name$ \\\hline
    1.  &&\\[11pt]
    2.  &&\\[11pt]
    3.  &&\\[11pt]\hline
\end{tabular}
\end{minipage}
\begin{minipage}{70mm}
\begin{tabular}{|l|l|}\hline
    $lang_part$ & $lang_total$ \\\hline\hline
    $lang_formal_assessment$ ($lang_max$. -10P) & \\[11pt]
    $lang_content_rating$ $if(points)$(max. $points$P)$endif$ & \\[11pt]\hline
    \textbf{$lang_in_total$} ($lang_min$. 0P) & \\[11pt]\hline
\end{tabular}
\end{minipage}

\bigskip
$lang_rated_by$:




# $lang_formal_assessment$

\begin{tabular}{|p{58mm}|l|p{93mm}|l|}\hline
    $lang_criteria$ & $lang_up_to$ & $lang_note$ & $lang_deductions$ \\\hline\hline
\end{tabular}

$lang_annotation$: $lang_cutoff_at$ 10P: max. -10P $lang_deduction$




# $lang_content_rating$

$if(questions)$
$for(questions)$

\begin{tabular}{|p{58mm}|p{107mm}|l|}\hline
    \textbf{$questions$} & $lang_note$ & $lang_points$ \\\hline\hline
\end{tabular}

$endfor$
$endif$







