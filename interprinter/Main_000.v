Inductive Bool : Set := false : Bool | true : Bool.

Inductive Bool_String : Set := nil : Bool_String | cons : Bool -> Bool_String -> Bool_String.

Inductive Nat : Set := zero : Nat | ones : Bool_String -> Nat.

Module Plus_000.

  Fixpoint function_0 (x : Bool_String) {struct x} : Bool_String
    :=
      match x with
        | nil => cons false nil
        | cons xp xs
          =>
            match xp with
              | false => cons true xs
              | true => cons false (function_0 xs)
            end
      end.

  Fixpoint function_1 (x y : Bool_String) (r : Bool) {struct x} : Bool_String
    :=
      match x, y with
        | nil, nil => cons r nil
        | nil, cons yp ys
          =>
            match r with
              | false => function_0 (cons yp ys)
              | true => cons yp (function_0 ys)
            end
        | cons xp xs, nil
          =>
            match r with
              | false => function_0 (cons xp xs)
              | true => cons xp (function_0 xs)
            end
        | cons xp xs, cons yp ys
          =>
            match xp, yp, r with
              | false, false, false => cons false (function_1 xs ys false)
              | false, false, true => cons true (function_1 xs ys false)
              | false, true, false => cons true (function_1 xs ys false)
              | false, true, true => cons false (function_1 xs ys true)
              | true, false, false => cons true (function_1 xs ys false)
              | true, false, true => cons false (function_1 xs ys true)
              | true, true, false => cons false (function_1 xs ys true)
              | true, true, true => cons true (function_1 xs ys true)
            end
      end.

  Definition function_2 (m n : Nat) : Nat
    :=
      match m, n with
        | zero, zero => zero
        | zero, ones n_ => ones n_
        | ones m_, zero => ones m_
        | ones m_, ones n_ => ones (function_1 m_ n_ false)
      end.

End Plus_000.

Definition plus (m n : Nat) : Nat := Plus_000.function_2 m n.
