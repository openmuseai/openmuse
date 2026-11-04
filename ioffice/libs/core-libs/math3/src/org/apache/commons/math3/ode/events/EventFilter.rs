// Java: import java.util.Arrays

// Java field: int HISTORY_SIZE
pub(crate) const HISTORY_SIZE: i32 = 0;

// Java: public class and
pub struct and {
}

// Java: public class EventFilter implements EventHandler
pub struct EventFilter {
    // Java field: EventHandler rawHandler
    pub(crate) rawHandler: Box<dyn std::any::Any>,
    // Java field: FilterType filter
    filter: Box<dyn std::any::Any>,
    // Java field: Transformer[] transformers
    pub(crate) transformers: Box<dyn std::any::Any>,
    // Java field: double[] updates
    updates: Vec<f64>,
    // Java field: boolean forward
    forward: bool,
    // Java field: double extremeT
    extremeT: f64,
}

// Java: public class and
impl and {
}

// Java: public class EventFilter implements EventHandler
impl EventFilter {
    // Java: EventFilter(final EventHandler rawHandler, final FilterType filter)
    pub fn EventFilter(&self) {
        // Java: this.rawHandler   = rawHandler;
        // Java: this.filter       = filter;
        // Java: this.transformers = new Transformer[HISTORY_SIZE];
        // Java: this.updates      = new double[HISTORY_SIZE];
    }

    // Java: void init(double t0, double[] y0, double t)
    pub fn init(&self) {
        // Java: // delegate to raw handler
        // Java: rawHandler.init(t0, y0, t);
        // Java: // initialize events triggering logic
        // Java: forward  = t >= t0;
        // Java: extremeT = forward ? Double.NEGATIVE_INFINITY : Double.POSITIVE_INFINITY;
        // Java: Arrays.fill(transformers, Transformer.UNINITIALIZED);
        // Java: Arrays.fill(updates, extremeT);
    }

    // Java: double g(double t, double[] y)
    pub fn g(&self) -> f64 {
        // Java: final double rawG = rawHandler.g(t, y);
        // Java: // search which transformer should be applied to g
        // Java: if (forward)
        {
            // Java: final int last = transformers.length - 1;
            // Java: if (extremeT < t)
            {
                // Java: // we are at the forward end of the history
                // Java: // check if a new rough root has been crossed
                // Java: final Transformer previous = transformers[last];
                // Java: final Transformer next     = filter.selectTransformer(previous, rawG, forward);
                // Java: if (next != previous)
                {
                    // Java: // there is a root somewhere between extremeT and t.
                    // Java: // the new transformer is valid for t (this is how we have just computed
                    // Java: // it above), but it is in fact valid on both sides of the root, so
                    // Java: // it was already valid before t and even up to previous time. We store
                    // Java: // the switch at extremeT for safety, to ensure the previous transformer
                    // Java: // is not applied too close of the root
                    // Java: System.arraycopy(updates,      1, updates,      0, last);
                    // Java: System.arraycopy(transformers, 1, transformers, 0, last);
                    // Java: updates[last]      = extremeT;
                    // Java: transformers[last] = next;
                }
                // Java: extremeT = t;
                // Java: // apply the transform
                // Java: return next.transformed(rawG);
            }
            // Java: else
            {
                // Java: // we are in the middle of the history
                // Java: // select the transformer
                // Java: for (int i = last; i > 0; --i)
                {
                    // Java: if (updates[i] <= t)
                    {
                        // Java: // apply the transform
                        // Java: return transformers[i].transformed(rawG);
                    }
                }
                // Java: return transformers[0].transformed(rawG);
            }
        }
        // Java: else
        {
            // Java: if (t < extremeT)
            {
                // Java: // we are at the backward end of the history
                // Java: // check if a new rough root has been crossed
                // Java: final Transformer previous = transformers[0];
                // Java: final Transformer next     = filter.selectTransformer(previous, rawG, forward);
                // Java: if (next != previous)
                {
                    // Java: // there is a root somewhere between extremeT and t.
                    // Java: // the new transformer is valid for t (this is how we have just computed
                    // Java: // it above), but it is in fact valid on both sides of the root, so
                    // Java: // it was already valid before t and even up to previous time. We store
                    // Java: // the switch at extremeT for safety, to ensure the previous transformer
                    // Java: // is not applied too close of the root
                    // Java: System.arraycopy(updates,      0, updates,      1, updates.length - 1);
                    // Java: System.arraycopy(transformers, 0, transformers, 1, transformers.length - 1);
                    // Java: updates[0]      = extremeT;
                    // Java: transformers[0] = next;
                }
                // Java: extremeT = t;
                // Java: // apply the transform
                // Java: return next.transformed(rawG);
            }
            // Java: else
            {
                // Java: // we are in the middle of the history
                // Java: // select the transformer
                // Java: for (int i = 0; i < updates.length - 1; ++i)
                {
                    // Java: if (t <= updates[i])
                    {
                        // Java: // apply the transform
                        // Java: return transformers[i].transformed(rawG);
                    }
                }
                // Java: return transformers[updates.length - 1].transformed(rawG);
            }
        }
        todo!()
    }

    // Java: Action eventOccurred(double t, double[] y, boolean increasing)
    pub fn eventOccurred(&self) -> Box<dyn std::any::Any> {
        // Java: // delegate to raw handler, fixing increasing status on the fly
        // Java: return rawHandler.eventOccurred(t, y, filter.getTriggeredIncreasing());
        todo!()
    }

    // Java: void resetState(double t, double[] y)
    pub fn resetState(&self) {
        // Java: // delegate to raw handler
        // Java: rawHandler.resetState(t, y);
    }

}

