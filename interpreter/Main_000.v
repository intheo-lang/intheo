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

Set Universe Polymorphism.

Module Finite_List_000.

  Inductive Tuple_0@{i | } : Type@{i} := tuple_0 : Tuple_0.

  Inductive Tuple_1@{i | } (A : Type@{i}) : Type@{i} := tuple_1 : A -> Tuple_1 A.

  Inductive Tuple_2@{i | } (A B : Type@{i}) : Type@{i} := tuple_2 : A -> B -> Tuple_2 A B.

  Inductive Tuple_3@{i | } (A B C : Type@{i}) : Type@{i} := tuple_3 : A -> B -> C -> Tuple_3 A B C.

  Fixpoint Type_0@{i | } (x : Bool_String) (A : Type@{i}) {struct x} : Type@{i}
    :=
      match x with
        | nil => Tuple_1@{i} A
        | cons xp xs
          =>
            match xp with
              | false => Tuple_2@{i} (Type_0 xs A) (Type_0 xs A)
              | true => Tuple_3@{i} A (Type_0 xs A) (Type_0 xs A)
            end
      end.

  Definition Type_1@{i | } (n : Nat) (A : Type@{i}) : Type@{i}
    :=
      match n with
        | zero => Tuple_0@{i}
        | ones n_ => Type_0@{i} n_ A
      end.

End Finite_List_000.

Definition Finite_List@{i | } (n : Nat) (A : Type@{i}) : Type@{i} := Finite_List_000.Type_1@{i} n A.
